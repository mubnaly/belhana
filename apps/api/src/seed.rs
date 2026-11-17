//! Idempotent seed data: product catalog, categories, units, recipes, batch
//! guides, user baseline prices, and the Olad Zain price post import.

use belhana_core::{catalog, rules};
use serde_json::json;
use sqlx::{Row, SqlitePool};

use crate::seed_data;

pub async fn run(pool: &SqlitePool) -> Result<(), belhana_common::CommonError> {
    seed_categories(pool).await?;
    seed_products(pool).await?;
    seed_recipes(pool).await?;
    seed_batch_guides(pool).await?;
    seed_baseline_prices(pool).await?;
    seed_olad_zain(pool).await?;
    seed_meal_tips(pool).await?;
    seed_rules_for_existing_households(pool).await?;
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

async fn seed_meal_tips(pool: &SqlitePool) -> Result<(), belhana_common::CommonError> {
    for (i, tip) in seed_data::MEAL_TIPS.iter().enumerate() {
        let exists: Option<String> = sqlx::query_scalar("SELECT id FROM meal_tips WHERE title_ar = ? AND is_system = 1")
            .bind(tip.title_ar)
            .fetch_optional(pool)
            .await
            .map_err(belhana_core::map_db)?;
        if exists.is_some() {
            continue;
        }
        sqlx::query(
            "INSERT INTO meal_tips (id, household_id, title_ar, title_en, category, body_ar, body_en, sort, is_system, created_at)
             VALUES (?, NULL, ?, ?, ?, ?, ?, ?, 1, datetime('now'))",
        )
        .append_id()
        .bind(tip.title_ar)
        .bind(tip.title_en)
        .bind(tip.category)
        .bind(tip.body_ar)
        .bind(tip.body_en)
        .bind(i as i64)
        .execute(pool)
        .await
        .map_err(belhana_core::map_db)?;
    }
    Ok(())
}

/// Default household rules requested by the family: an instruction sentence for
/// the assistant, staple foods by meal period, a fixed spice budget, and
/// explicit next-week carry-over. Idempotent and marked `system-default` so the
/// user can edit or delete them without them reappearing.
pub async fn seed_household_rules(
    pool: &SqlitePool,
    household_id: &str,
) -> Result<(), belhana_common::CommonError> {
    let rice = catalog::find_by_arabic_name(pool, "أرز").await?.map(|p| p.id);
    let bread = catalog::find_by_arabic_name(pool, "خبز بلدي").await?.map(|p| p.id);
    let salt = catalog::find_by_arabic_name(pool, "ملح").await?.map(|p| p.id);

    let mut defs: Vec<(&str, serde_json::Value)> = vec![(
        "ai_instruction",
        json!({"sentence": "فكّر في بقايا المخزن أولًا، واجعل الأرز غداءً معتادًا والخبز أساس الفطور والعشاء، واشترِ التوابل بمبلغ محدد مثل 20 جنيهًا من الملح بدل شراء الكيلو."}),
    )];
    if let Some(id) = rice {
        defs.push(("meal_staple", json!({"period": "lunch", "product_id": id})));
    }
    if let Some(id) = bread {
        defs.push(("meal_staple", json!({"period": "breakfast", "product_id": id})));
        defs.push(("meal_staple", json!({"period": "dinner", "product_id": id})));
    }
    if let Some(id) = salt {
        defs.push(("fixed_spend", json!({"product_id": id, "amount_minor": 2000})));
    }
    defs.push(("leftover_carryover", json!({"carry_over": true})));

    for (rule_type, params) in defs {
        let params_text = params.to_string();
        let exists: Option<String> = sqlx::query_scalar(
            "SELECT id FROM household_rules
             WHERE household_id = ? AND rule_type = ? AND params = ? AND notes = 'system-default'",
        )
        .bind(household_id)
        .bind(rule_type)
        .bind(&params_text)
        .fetch_optional(pool)
        .await
        .map_err(belhana_core::map_db)?;
        if exists.is_some() {
            continue;
        }
        rules::create_rule(
            pool,
            household_id,
            &rules::CreateRule {
                rule_type: rule_type.to_string(),
                priority: "soft".to_string(),
                params,
                notes: Some("system-default".to_string()),
            },
        )
        .await?;
    }
    Ok(())
}

async fn seed_rules_for_existing_households(pool: &SqlitePool) -> Result<(), belhana_common::CommonError> {
    let ids: Vec<String> = sqlx::query_scalar("SELECT id FROM households")
        .fetch_all(pool)
        .await
        .map_err(belhana_core::map_db)?;
    for id in ids {
        seed_household_rules(pool, &id).await?;
    }
    Ok(())
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
