//! Approximate calorie estimates from the product catalog.
//!
//! The catalog stores an editorial kcal value per 100 g (or 100 ml) plus the
//! typical grams in one default unit. These are deliberately coarse estimates
//! for everyday planning; the UI labels them as such and they are never
//! presented as verified laboratory data.

use std::collections::HashMap;

use belhana_common::CommonError;
use serde::Serialize;
use sqlx::sqlite::SqliteRow;
use sqlx::{Row, SqlitePool};

use crate::map_db;

/// Typical edible grams for units that are not mass/volume.
fn generic_unit_grams(unit: &str) -> Option<f64> {
    match unit {
        "piece" => Some(100.0),
        "package" => Some(250.0),
        "bundle" => Some(100.0),
        "carton" => Some(600.0),
        "dozen" => Some(600.0),
        "pair" => Some(500.0),
        "plate" => Some(300.0),
        "cup" => Some(200.0),
        "tbsp" => Some(15.0),
        "tsp" => Some(5.0),
        _ => None,
    }
}

/// Converts a quantity in `unit` to grams, using the product's catalog weight
/// for its default unit (and package size when a smaller unit is used).
pub fn grams_for(
    quantity: f64,
    unit: &str,
    grams_per_unit: Option<f64>,
    default_unit: Option<&str>,
    package_size: Option<f64>,
    package_unit: Option<&str>,
) -> Option<f64> {
    if quantity <= 0.0 {
        return Some(0.0);
    }
    let unit = unit.trim();
    let per_unit = match unit {
        "kg" | "l" => 1000.0,
        "g" | "ml" => 1.0,
        _ => {
            if let (Some(grams), Some(default_unit)) = (grams_per_unit, default_unit) {
                if unit == default_unit {
                    grams
                } else if let (Some(size), Some(package_unit)) = (package_size, package_unit) {
                    if size > 0.0 && unit == package_unit {
                        // e.g. a carton of 30 eggs → one piece is carton_grams / 30.
                        grams / size
                    } else {
                        generic_unit_grams(unit)?
                    }
                } else {
                    generic_unit_grams(unit)?
                }
            } else {
                generic_unit_grams(unit)?
            }
        }
    };
    Some(quantity * per_unit)
}

/// Calories for a quantity of a product, or `None` when nutrition is unknown.
pub fn kcal_for(
    quantity: f64,
    unit: &str,
    kcal_per_100g: Option<f64>,
    grams_per_unit: Option<f64>,
    default_unit: Option<&str>,
    package_size: Option<f64>,
    package_unit: Option<&str>,
) -> Option<i64> {
    let kcal_per_100g = kcal_per_100g?;
    let grams = grams_for(
        quantity,
        unit,
        grams_per_unit,
        default_unit,
        package_size,
        package_unit,
    )?;
    Some((grams * kcal_per_100g / 100.0).round() as i64)
}

/// Reads the nutrition columns from a joined row and returns the calories for
/// the given quantity/unit. Missing columns behave as unknown nutrition.
pub fn kcal_from_row(row: &SqliteRow, quantity: f64, unit: &str) -> Option<i64> {
    let kcal_per_100g = row.try_get::<Option<f64>, _>("kcal_per_100g").ok().flatten();
    let grams_per_unit = row.try_get::<Option<f64>, _>("grams_per_unit").ok().flatten();
    let default_unit = row.try_get::<Option<String>, _>("default_unit").ok().flatten();
    let package_size = row.try_get::<Option<f64>, _>("package_size").ok().flatten();
    let package_unit = row.try_get::<Option<String>, _>("package_unit").ok().flatten();
    kcal_for(
        quantity,
        unit,
        kcal_per_100g,
        grams_per_unit,
        default_unit.as_deref(),
        package_size,
        package_unit.as_deref(),
    )
}

/// Estimated calories per serving for every active recipe, computed from its
/// ingredient quantities and the catalog nutrition values.
pub async fn recipe_kcal_map(pool: &SqlitePool) -> Result<HashMap<String, i64>, CommonError> {
    let rows = sqlx::query(
        "SELECT ri.recipe_id, ri.quantity, ri.unit, r.default_servings,
                p.kcal_per_100g, p.grams_per_unit, p.default_unit, p.package_size, p.package_unit
         FROM recipe_ingredients ri
         JOIN recipes r ON r.id = ri.recipe_id
         LEFT JOIN products p ON p.id = ri.product_id
         WHERE r.is_active = 1",
    )
    .fetch_all(pool)
    .await
    .map_err(map_db)?;

    let mut totals: HashMap<String, (f64, i64)> = HashMap::new();
    for row in rows {
        let recipe_id: String = row.get("recipe_id");
        let servings: i64 = row.get::<i64, _>("default_servings").max(1);
        let quantity: f64 = row.get("quantity");
        let unit: String = row.get("unit");
        if let Some(kcal) = kcal_from_row(&row, quantity, &unit) {
            let entry = totals.entry(recipe_id).or_insert((0.0, servings));
            entry.0 += kcal as f64;
        }
    }

    Ok(totals
        .into_iter()
        .map(|(id, (total, servings))| (id, (total / servings as f64).round() as i64))
        .collect())
}

/// Estimated macronutrients for a quantity of food. All fields are optional:
/// a product may have calories but no macro data, and every value is an
/// editorial estimate rather than a laboratory measurement.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MacroEstimate {
    pub kcal: Option<i64>,
    pub protein_g: Option<f64>,
    pub carbs_g: Option<f64>,
    pub fat_g: Option<f64>,
    pub fiber_g: Option<f64>,
}

fn round1(v: f64) -> f64 {
    (v * 10.0).round() / 10.0
}

fn cell(row: &SqliteRow, name: &str) -> Option<f64> {
    row.try_get::<Option<f64>, _>(name).ok().flatten()
}

/// Reads macro columns from a joined row and scales them to `quantity`/`unit`.
pub fn macro_from_row(row: &SqliteRow, quantity: f64, unit: &str) -> Option<MacroEstimate> {
    let per_100 = (
        cell(row, "kcal_per_100g"),
        cell(row, "protein_per_100g"),
        cell(row, "carbs_per_100g"),
        cell(row, "fat_per_100g"),
        cell(row, "fiber_per_100g"),
    );
    if per_100.0.is_none()
        && per_100.1.is_none()
        && per_100.2.is_none()
        && per_100.3.is_none()
        && per_100.4.is_none()
    {
        return None;
    }
    let grams = grams_for(
        quantity,
        unit,
        cell(row, "grams_per_unit"),
        row.try_get::<Option<String>, _>("default_unit")
            .ok()
            .flatten()
            .as_deref(),
        cell(row, "package_size"),
        row.try_get::<Option<String>, _>("package_unit")
            .ok()
            .flatten()
            .as_deref(),
    )?;
    let factor = grams / 100.0;
    let scale = |v: Option<f64>| v.map(|v| round1(v * factor));
    Some(MacroEstimate {
        kcal: per_100.0.map(|v| (v * factor).round() as i64),
        protein_g: scale(per_100.1),
        carbs_g: scale(per_100.2),
        fat_g: scale(per_100.3),
        fiber_g: scale(per_100.4),
    })
}

/// Estimated macros per serving for every active recipe, computed from the
/// ingredient quantities and the catalog macro columns.
pub async fn recipe_macro_map(pool: &SqlitePool) -> Result<HashMap<String, MacroEstimate>, CommonError> {
    let rows = sqlx::query(
        "SELECT ri.recipe_id, ri.quantity, ri.unit, r.default_servings,
                p.kcal_per_100g, p.protein_per_100g, p.carbs_per_100g, p.fat_per_100g,
                p.fiber_per_100g, p.grams_per_unit, p.default_unit, p.package_size, p.package_unit
         FROM recipe_ingredients ri
         JOIN recipes r ON r.id = ri.recipe_id
         LEFT JOIN products p ON p.id = ri.product_id
         WHERE r.is_active = 1",
    )
    .fetch_all(pool)
    .await
    .map_err(map_db)?;

    #[derive(Default)]
    struct Sums {
        servings: i64,
        kcal: f64,
        protein: f64,
        carbs: f64,
        fat: f64,
        fiber: f64,
        has_kcal: bool,
        has_protein: bool,
        has_carbs: bool,
        has_fat: bool,
        has_fiber: bool,
    }
    let mut totals: HashMap<String, Sums> = HashMap::new();
    for row in rows {
        let recipe_id: String = row.get("recipe_id");
        let servings = row.get::<i64, _>("default_servings").max(1);
        let quantity: f64 = row.get("quantity");
        let unit: String = row.get("unit");
        if let Some(m) = macro_from_row(&row, quantity, &unit) {
            let s = totals.entry(recipe_id).or_default();
            s.servings = servings;
            if let Some(v) = m.kcal {
                s.kcal += v as f64;
                s.has_kcal = true;
            }
            if let Some(v) = m.protein_g {
                s.protein += v;
                s.has_protein = true;
            }
            if let Some(v) = m.carbs_g {
                s.carbs += v;
                s.has_carbs = true;
            }
            if let Some(v) = m.fat_g {
                s.fat += v;
                s.has_fat = true;
            }
            if let Some(v) = m.fiber_g {
                s.fiber += v;
                s.has_fiber = true;
            }
        }
    }
    Ok(totals
        .into_iter()
        .map(|(id, s)| {
            let d = s.servings.max(1) as f64;
            (
                id,
                MacroEstimate {
                    kcal: if s.has_kcal {
                        Some((s.kcal / d).round() as i64)
                    } else {
                        None
                    },
                    protein_g: s.has_protein.then(|| round1(s.protein / d)),
                    carbs_g: s.has_carbs.then(|| round1(s.carbs / d)),
                    fat_g: s.has_fat.then(|| round1(s.fat / d)),
                    fiber_g: s.has_fiber.then(|| round1(s.fiber / d)),
                },
            )
        })
        .collect())
}

/// Energy burned from a Metabolic Equivalent of Task value:
/// `kcal = MET × kg × hours`. MET values are approximations and vary by person.
pub fn met_burn_kcal(met: f64, weight_kg: f64, minutes: f64) -> Option<i64> {
    if !met.is_finite() || !weight_kg.is_finite() || !minutes.is_finite() {
        return None;
    }
    if met <= 0.0 || weight_kg <= 0.0 || minutes < 0.0 {
        return None;
    }
    Some((met * weight_kg * minutes / 60.0).round() as i64)
}

/// Mifflin–St Jeor basal metabolic rate (kcal/day). Estimates only.
pub fn bmr_mifflin(sex: &str, weight_kg: f64, height_cm: f64, age: f64) -> Option<f64> {
    let valid = weight_kg > 0.0 && height_cm > 0.0 && age > 0.0;
    if !valid {
        return None;
    }
    let base = 10.0 * weight_kg + 6.25 * height_cm - 5.0 * age;
    match sex {
        "male" => Some(base + 5.0),
        "female" => Some(base - 161.0),
        _ => None,
    }
}

/// Conventional activity multipliers applied to BMR for a daily estimate.
pub fn activity_factor(key: &str) -> Option<f64> {
    match key {
        "sedentary" => Some(1.2),
        "light" => Some(1.375),
        "moderate" => Some(1.55),
        "active" => Some(1.725),
        "very_active" => Some(1.9),
        _ => None,
    }
}

/// Splits remaining energy (after the protein target) into carbs and fat using
/// a 62%/38% split. A coaching estimate, not a prescription.
pub fn macro_targets(target_kcal: i64, protein_g: f64) -> (f64, f64) {
    let remaining = (target_kcal as f64 - protein_g * 4.0).max(0.0);
    (round1(remaining * 0.62 / 4.0), round1(remaining * 0.38 / 9.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grams_for_known_units() {
        assert_eq!(grams_for(1.5, "kg", None, None, None, None), Some(1500.0));
        assert_eq!(
            grams_for(2.0, "piece", Some(50.0), Some("piece"), None, None),
            Some(100.0)
        );
        assert_eq!(grams_for(0.0, "kg", None, None, None, None), Some(0.0));
    }

    #[test]
    fn kcal_for_scales_per_100g() {
        assert_eq!(
            kcal_for(200.0, "g", Some(250.0), None, None, None, None),
            Some(500)
        );
        assert_eq!(kcal_for(100.0, "g", None, None, None, None, None), None);
    }

    #[test]
    fn met_burn_uses_typical_examples() {
        // 3.5 METs × 70 kg × 1 h ≈ 245 kcal (brisk walk or moderate lifting).
        assert_eq!(met_burn_kcal(3.5, 70.0, 60.0), Some(245));
        assert_eq!(met_burn_kcal(6.0, 70.0, 60.0), Some(420));
        assert_eq!(met_burn_kcal(0.0, 70.0, 60.0), None);
    }

    #[test]
    fn bmr_and_macro_targets_are_rounded() {
        let male = bmr_mifflin("male", 70.0, 175.0, 30.0).unwrap();
        let female = bmr_mifflin("female", 70.0, 175.0, 30.0).unwrap();
        assert!((male - 1648.75).abs() < 0.01);
        assert!((female - 1482.75).abs() < 0.01);
        assert_eq!(activity_factor("moderate"), Some(1.55));
        let (carbs, fat) = macro_targets(2000, 120.0);
        assert!(carbs > 0.0 && fat > 0.0);
    }
}
