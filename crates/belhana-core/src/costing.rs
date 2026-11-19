//! Deterministic recipe costing from selected price observations.
//!
//! Rules:
//! - Cost is computed from a *selected price observation* with a resolved unit.
//! - Missing price or unconvertible unit → the component is reported as
//!   unknown; no precise total is fabricated for incomplete costs.
//! - Raw recipe cost, cost per portion, and inventory-adjusted cash outlay are
//!   reported separately.

use belhana_common::{CommonError, Money};
use serde::Serialize;
use sqlx::SqlitePool;

use belhana_common::units::universal_convert_str;
use crate::pricing::PriceObservation;
use crate::{catalog, pricing, recipes};

#[derive(Debug, Serialize)]
pub struct CostLine {
    pub product_id: String,
    pub name_ar: String,
    pub quantity: f64,
    pub unit: String,
    pub price_observation_id: Option<String>,
    pub price_minor: Option<i64>,      // total for this line; None when unknown
    pub price_basis: Option<String>,   // description of the price basis
    pub freshness: Option<String>,
    pub known: bool,
}

#[derive(Debug, Serialize)]
pub struct RecipeCost {
    pub recipe_id: String,
    pub servings: i64,
    /// Cost of lines with known prices only. `None` when any line is unknown.
    pub total_minor: Option<i64>,
    pub per_serving_minor: Option<i64>,
    pub known_total_minor: i64,
    pub lines: Vec<CostLine>,
    pub unknown_count: i64,
}

/// Price per single unit of `target_unit` derived from an observation.
/// Returns the per-unit amount in minor units.
/// Shared serving scale. Mirrors the household-member count used by the UI:
/// `servings / default_servings`, with a safe fallback for legacy recipes.
pub fn scale_factor(default_servings: i64, servings: i64) -> f64 {
    if default_servings > 0 {
        servings as f64 / default_servings as f64
    } else {
        1.0
    }
}

pub fn price_per_unit(obs: &PriceObservation, target_unit: &str) -> Option<f64> {
    let amount = obs.amount_minor? as f64;
    let unit = obs.unit.as_deref()?;
    let size = obs.package_size.unwrap_or(1.0).max(0.0001);
    if unit == target_unit {
        return Some(amount / size);
    }
    // e.g. carton of 30 eggs: package_unit = piece → price per piece.
    if obs.package_unit.as_deref() == Some(target_unit) {
        return Some(amount / size);
    }
    // Same-dimension conversion (kg↔g, l↔ml, dozen↔piece).
    let conv = universal_convert_str(1.0, target_unit, unit)?;
    Some(conv * amount / size)
}

pub async fn compute_recipe_cost(
    pool: &SqlitePool,
    recipe_id: &str,
    servings: i64,
    policy: &str,
) -> Result<RecipeCost, CommonError> {
    let (recipe, ingredients, _) = recipes::get_recipe(pool, recipe_id).await?;
    let scale = scale_factor(recipe.default_servings, servings);
    let mut lines = Vec::new();
    let mut known_total = 0i64;
    let mut unknown_count = 0i64;

    for ing in &ingredients {
        let name = ing.raw_name_ar.clone().unwrap_or_else(|| "—".into());
        let qty = ing.quantity * scale;
        let Some(pid) = &ing.product_id else {
            lines.push(CostLine {
                product_id: String::new(),
                name_ar: name,
                quantity: qty,
                unit: ing.unit.clone(),
                price_observation_id: None,
                price_minor: None,
                price_basis: None,
                freshness: None,
                known: false,
            });
            unknown_count += 1;
            continue;
        };
        let product = catalog::get_product(pool, pid).await?;
        let obs = pricing::best_observation_for_product(pool, pid, policy).await?;
        let (price, basis, obs_id, fresh) = match &obs {
            Some(o) => match price_per_unit(o, &ing.unit) {
                Some(per_unit) => {
                    let total = (qty * per_unit).round() as i64;
                    (
                        Some(total),
                        Some(format!(
                            "{} EGP / {} {}",
                            Money::egp(o.amount_minor.unwrap_or(0)).major_f64(),
                            o.package_size.unwrap_or(1.0),
                            o.unit.clone().unwrap_or_default()
                        )),
                        Some(o.id.clone()),
                        Some(pricing::freshness(o.observation_date.as_deref(), pricing::is_user_confirmed(&o.review_status)).to_string()),
                    )
                }
                None => (None, Some("unit not convertible".into()), Some(o.id.clone()), None),
            },
            None => (None, Some("no usable price".into()), None, None),
        };
        if price.is_some() {
            known_total += price.unwrap();
        } else {
            unknown_count += 1;
        }
        lines.push(CostLine {
            product_id: product.id,
            name_ar: product.name_ar,
            quantity: qty,
            unit: ing.unit.clone(),
            price_observation_id: obs_id,
            price_minor: price,
            price_basis: basis,
            freshness: fresh,
            known: price.is_some(),
        });
    }

    let total = if unknown_count == 0 { Some(known_total) } else { None };
    let per_serving = total.map(|t| Money::egp(t).scale_ratio(1, servings.max(1)).minor);
    Ok(RecipeCost {
        recipe_id: recipe.id.to_string(),
        servings,
        total_minor: total,
        per_serving_minor: per_serving,
        known_total_minor: known_total,
        lines,
        unknown_count,
    })
}

/// Raw ingredient totals (no inventory deduction) as a simple map, used by the
/// planner and shopping aggregation.
pub async fn ingredient_quantities(
    pool: &SqlitePool,
    recipe_id: &str,
    servings: i64,
) -> Result<Vec<(String, f64, String)>, CommonError> {
    let (recipe, ingredients, _) = recipes::get_recipe(pool, recipe_id).await?;
    let scale = scale_factor(recipe.default_servings, servings);
    Ok(ingredients
        .iter()
        .filter_map(|ing| ing.product_id.clone().map(|pid| (pid, ing.quantity * scale, ing.unit.clone())))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pricing::PriceObservation;

    fn obs(amount_minor: i64, unit: &str, size: Option<f64>, package_unit: Option<&str>) -> PriceObservation {
        PriceObservation {
            id: "o".into(),
            product_id: Some("p".into()),
            product_variant: None,
            merchant_id: None,
            location_id: None,
            amount_minor: Some(amount_minor),
            currency: "EGP".into(),
            unit: Some(unit.into()),
            unit_status: "confirmed".into(),
            package_size: size,
            package_unit: package_unit.map(|s| s.to_string()),
            observation_date: None,
            submitted_at: String::new(),
            source_type: "manual".into(),
            source_url: None,
            raw_text: None,
            raw_product_label: None,
            confidence: None,
            review_status: "verified".into(),
            previous_amount_minor: None,
            promotion_note: None,
            reviewed_by: None,
            reviewed_at: None,
            batch_id: None,
            baseline_owner_user_id: None,
            occurrence_index: 0,
            notes: None,
        }
    }

    #[test]
    fn package_price_is_divided_by_pack_size() {
        // 30 eggs for 130 EGP → 433.33 piasters per piece.
        let o = obs(13000, "piece", Some(30.0), Some("piece"));
        let per = price_per_unit(&o, "piece").expect("resolved");
        assert!((per - 13000.0 / 30.0).abs() < 0.001, "per=$per");
    }

    #[test]
    fn package_unit_is_used_when_observation_unit_differs() {
        // A carton observation whose package_unit is 'piece' resolves to per-piece.
        let o = obs(6500, "carton", Some(15.0), Some("piece"));
        let per = price_per_unit(&o, "piece").expect("resolved");
        assert!((per - 6500.0 / 15.0).abs() < 0.001, "per=$per");
    }

    #[test]
    fn container_units_do_not_convert_universally() {
        // carton has no universal conversion to piece; without package metadata
        // the comparison must stay unresolved rather than guessing.
        let o = obs(13000, "carton", None, None);
        assert!(price_per_unit(&o, "piece").is_none());
    }

    #[test]
    fn servings_scale_matches_household_count() {
        // A 4-serving recipe doubles for 8 family members and halves for 2.
        assert!((scale_factor(4, 8) - 2.0).abs() < 1e-9);
        assert!((scale_factor(4, 2) - 0.5).abs() < 1e-9);
        assert!((scale_factor(4, 4) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn legacy_zero_default_servings_do_not_divide_by_zero() {
        assert!((scale_factor(0, 6) - 1.0).abs() < 1e-9);
    }
}
