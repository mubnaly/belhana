//! Idempotent seed data: product catalog, categories, units, recipes, batch
//! guides, user baseline prices, and the Olad Zain price post import.

use belhana_core::catalog;
use sqlx::{Row, SqlitePool};

use crate::seed_data;

pub async fn run(pool: &SqlitePool) -> Result<(), belhana_common::CommonError> {
    seed_categories(pool).await?;
    seed_products(pool).await?;
    seed_recipes(pool).await?;
    seed_batch_guides(pool).await?;
    seed_baseline_prices(pool).await?;
    seed_olad_zain(pool).await?;
    Ok(())
}

async fn seed_categories(pool: &SqlitePool) -> Result<(), belhana_common::CommonError> {
    let cats = seed_data::CATEGORIES;
    for (slug, name_ar, name_en, sort) in cats {
        let exists: Option<String> =
            sqlx::query_scalar("SELECT id FROM product_categories WHERE slug = ?")
                .bind(slug)
                .fetch_optional(pool)
                .await
                .map_err(belhana_core::map_db)?;
        if exists.is_some() {
            continue;
        }
        sqlx::query("INSERT INTO product_categories (id, slug, name_ar, name_en, sort) VALUES (?, ?, ?, ?, ?)")
            .append_id()
            .bind(slug)
            .bind(name_ar)
            .bind(name_en)
            .bind(sort)
            .execute(pool)
            .await
            .map_err(belhana_core::map_db)?;
    }
    Ok(())
}

async fn seed_products(pool: &SqlitePool) -> Result<(), belhana_common::CommonError> {
    let cats: Vec<(String, String)> = sqlx::query("SELECT slug, id FROM product_categories")
        .fetch_all(pool)
        .await
        .map_err(belhana_core::map_db)?
        .into_iter()
        .map(|r| (r.get::<String, _>("slug"), r.get::<String, _>("id")))
        .collect();
    for p in seed_data::PRODUCTS {
        if catalog::find_by_arabic_name(pool, p.name_ar).await?.is_some() {
            continue;
        }
        let category_id = cats
            .iter()
            .find(|(slug, _)| slug == p.category)
            .map(|(_, id)| id.clone());
        catalog::create_product(
            pool,
            &catalog::CreateProduct {
                category_id,
                name_ar: p.name_ar.to_string(),
                name_en: p.name_en.map(|s| s.to_string()),
                default_unit: p.default_unit.map(|s| s.to_string()),
                comparison_unit: None,
                package_size: p.package_size,
                package_unit: p.package_unit.map(|s| s.to_string()),
                brand: None,
                aliases: p
                    .aliases
                    .as_ref()
                    .map(|v| v.iter().map(|s| s.to_string()).collect()),
                notes: None,
            },
        )
        .await?;
    }
    Ok(())
}

async fn seed_recipes(pool: &SqlitePool) -> Result<(), belhana_common::CommonError> {
    for r in seed_data::RECIPES {
        let exists: Option<String> = sqlx::query_scalar("SELECT id FROM recipes WHERE name_ar = ? AND is_system = 1")
            .bind(r.name_ar)
            .fetch_optional(pool)
            .await
            .map_err(belhana_core::map_db)?;
        if exists.is_some() {
            continue;
        }
        // map ingredient product names to catalog products
        let mut ingredients = Vec::new();
        for ing in r.ingredients {
            let product_id = catalog::find_by_arabic_name(pool, ing.0).await?.map(|p| p.id);
            ingredients.push(belhana_core::recipes::CreateIngredient {
                product_id,
                raw_name_ar: Some(ing.0.to_string()),
                raw_name_en: None,
                quantity: ing.1,
                unit: ing.2.to_string(),
                optional: Some(false),
                note: None,
            });
        }
        belhana_core::recipes::create_recipe(
            pool,
            None,
            None,
            &belhana_core::recipes::CreateRecipe {
                name_ar: r.name_ar.to_string(),
                name_en: Some(r.name_en.to_string()),
                description: Some(r.description.to_string()),
                cuisine: Some("egyptian".into()),
                meal_types: r.meal_types.iter().map(|s| s.to_string()).collect(),
                difficulty: Some(r.difficulty.to_string()),
                prep_minutes: Some(r.prep),
                cook_minutes: Some(r.cook),
                default_servings: Some(r.servings),
                cooking_method: Some(r.method.to_string()),
                equipment: Some(Vec::new()),
                allergens: Some(r.allergens.iter().map(|s| s.to_string()).collect()),
                storage_notes: None,
                source_type: Some("original".into()),
                source_ref: Some("Belhana seed library (original content, written for Belhana)".into()),
                tags: Some(r.tags.iter().map(|s| s.to_string()).collect()),
                ingredients,
                steps: r
                    .steps
                    .iter()
                    .map(|s| belhana_core::recipes::CreateStep {
                        text_ar: s.to_string(),
                        text_en: None,
                        timer_minutes: None,
                    })
                    .collect(),
            },
        )
        .await?;
    }
    Ok(())
}

async fn seed_batch_guides(pool: &SqlitePool) -> Result<(), belhana_common::CommonError> {
    for g in seed_data::BATCH_GUIDES {
        let exists: Option<String> = sqlx::query_scalar("SELECT id FROM batch_guides WHERE title_ar = ? AND is_system = 1")
            .bind(g.title_ar)
            .fetch_optional(pool)
            .await
            .map_err(belhana_core::map_db)?;
        if exists.is_some() {
            continue;
        }
        sqlx::query(
            "INSERT INTO batch_guides (id, household_id, title_ar, title_en, category, batch_size_text, ingredients, steps, portions, storage, safety_source, safety_source_date, is_system, created_at)
             VALUES (?, NULL, ?, ?, ?, ?, ?, ?, ?, ?, 'Belhana editorial review of current public food-safety guidance', '2026-11-01', 1, datetime('now'))",
        )
        .append_id()
        .bind(g.title_ar)
        .bind(g.title_en)
        .bind(g.category)
        .bind(g.batch_size_text)
        .bind(g.ingredients)
        .bind(g.steps)
        .bind(g.portions)
        .bind(g.storage)
        .execute(pool)
        .await
        .map_err(belhana_core::map_db)?;
    }
    Ok(())
}

async fn seed_baseline_prices(pool: &SqlitePool) -> Result<(), belhana_common::CommonError> {
    // Seed once: marker = any baseline observation exists.
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM price_observations WHERE source_type = 'user_baseline'")
        .fetch_one(pool)
        .await
        .map_err(belhana_core::map_db)?;
    if count > 0 {
        return Ok(());
    }
    for b in seed_data::BASELINES {
        let (product, amount_minor, unit, _note) = b;
        if let Some(p) = catalog::find_by_arabic_name(pool, product).await? {
            belhana_core::pricing::create_baseline_estimate(
                pool,
                "system-seed",
                &p.id,
                *amount_minor,
                unit,
                Some(belhana_core::today().format("%Y-%m-%d").to_string()),
            )
            .await?;
        }
    }
    Ok(())
}

async fn seed_olad_zain(pool: &SqlitePool) -> Result<(), belhana_common::CommonError> {
    belhana_core::imports::seed_olad_zain(pool, seed_data::OLAD_ZAIN_POST).await
}

// tiny extension to avoid repeating new_id() everywhere
trait AppendId<'q> {
    fn append_id(self) -> sqlx::query::Query<'q, sqlx::Sqlite, sqlx::sqlite::SqliteArguments<'q>>;
}
impl<'q> AppendId<'q> for sqlx::query::Query<'q, sqlx::Sqlite, sqlx::sqlite::SqliteArguments<'q>> {
    fn append_id(self) -> Self {
        self.bind(belhana_core::new_id())
    }
}
