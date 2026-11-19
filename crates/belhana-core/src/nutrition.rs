//! Approximate calorie estimates from the product catalog.
//!
//! The catalog stores an editorial kcal value per 100 g (or 100 ml) plus the
//! typical grams in one default unit. These are deliberately coarse estimates
//! for everyday planning; the UI labels them as such and they are never
//! presented as verified laboratory data.

use std::collections::HashMap;

use belhana_common::CommonError;
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
