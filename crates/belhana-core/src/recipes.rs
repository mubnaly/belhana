//! Recipe catalog and economical batch-preparation guides.

use belhana_common::CommonError;
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};

use crate::{map_db, new_id, now_rfc3339};

#[derive(Debug, Clone, Serialize)]
pub struct Recipe {
    pub id: String,
    pub household_id: Option<String>,
    pub name_ar: String,
    pub name_en: Option<String>,
    pub description: Option<String>,
    pub cuisine: String,
    pub meal_types: Vec<String>,
    pub difficulty: String,
    pub prep_minutes: i64,
    pub cook_minutes: i64,
    pub default_servings: i64,
    pub cooking_method: Option<String>,
    pub equipment: Vec<String>,
    pub allergens: Vec<String>,
    pub storage_notes: Option<String>,
    pub source_type: String,
    pub source_ref: Option<String>,
    pub child_portion_factor: f64,
    pub is_system: bool,
    pub tags: Vec<String>,
    /// Estimated calories for one serving (editorial catalog estimate).
    pub kcal_per_serving: Option<i64>,
    /// Estimated macros for one serving.
    pub macros_per_serving: Option<crate::nutrition::MacroEstimate>,
}

fn parse_json_array(s: Option<&String>) -> Vec<String> {
    s.and_then(|v| serde_json::from_str::<Vec<String>>(v).ok())
        .unwrap_or_default()
}

impl From<sqlx::sqlite::SqliteRow> for Recipe {
    fn from(r: sqlx::sqlite::SqliteRow) -> Self {
        Recipe {
            id: r.get("id"),
            household_id: r.get("household_id"),
            name_ar: r.get("name_ar"),
            name_en: r.get("name_en"),
            description: r.get("description"),
            cuisine: r.get("cuisine"),
            meal_types: parse_json_array(r.get::<Option<String>, _>("meal_types").as_ref()),
            difficulty: r.get("difficulty"),
            prep_minutes: r.get("prep_minutes"),
            cook_minutes: r.get("cook_minutes"),
            default_servings: r.get("default_servings"),
            cooking_method: r.get("cooking_method"),
            equipment: parse_json_array(r.get::<Option<String>, _>("equipment").as_ref()),
            allergens: parse_json_array(r.get::<Option<String>, _>("allergens").as_ref()),
            storage_notes: r.get("storage_notes"),
            source_type: r.get("source_type"),
            source_ref: r.get("source_ref"),
            child_portion_factor: r.get("child_portion_factor"),
            is_system: r.get::<i64, _>("is_system") != 0,
            tags: Vec::new(),
            kcal_per_serving: None,
            macros_per_serving: None,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct RecipeIngredient {
    pub id: String,
    pub recipe_id: String,
    pub product_id: Option<String>,
    pub raw_name_ar: Option<String>,
    pub raw_name_en: Option<String>,
    pub quantity: f64,
    pub unit: String,
    pub optional: bool,
    pub note: Option<String>,
    /// Estimated calories for this listed quantity, when known.
    pub kcal: Option<i64>,
    /// Estimated macros for this listed quantity.
    pub macros: Option<crate::nutrition::MacroEstimate>,
}

impl From<sqlx::sqlite::SqliteRow> for RecipeIngredient {
    fn from(r: sqlx::sqlite::SqliteRow) -> Self {
        RecipeIngredient {
            id: r.get("id"),
            recipe_id: r.get("recipe_id"),
            product_id: r.get("product_id"),
            raw_name_ar: r.get("raw_name_ar"),
            raw_name_en: r.get("raw_name_en"),
            quantity: r.get("quantity"),
            unit: r.get("unit"),
            optional: r.get::<i64, _>("optional") != 0,
            note: r.get("note"),
            kcal: None,
            macros: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct RecipeStep {
    pub step_number: i64,
    pub text_ar: String,
    pub text_en: Option<String>,
    pub timer_minutes: Option<i64>,
}

impl From<sqlx::sqlite::SqliteRow> for RecipeStep {
    fn from(r: sqlx::sqlite::SqliteRow) -> Self {
        RecipeStep {
            step_number: r.get("step_number"),
            text_ar: r.get("text_ar"),
            text_en: r.get("text_en"),
            timer_minutes: r.get("timer_minutes"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateRecipe {
    pub name_ar: String,
    pub name_en: Option<String>,
    pub description: Option<String>,
    pub cuisine: Option<String>,
    pub meal_types: Vec<String>,
    pub difficulty: Option<String>,
    pub prep_minutes: Option<i64>,
    pub cook_minutes: Option<i64>,
    pub default_servings: Option<i64>,
    pub cooking_method: Option<String>,
    pub equipment: Option<Vec<String>>,
    pub allergens: Option<Vec<String>>,
    pub storage_notes: Option<String>,
    pub source_type: Option<String>,
    pub source_ref: Option<String>,
    pub tags: Option<Vec<String>>,
    pub ingredients: Vec<CreateIngredient>,
    pub steps: Vec<CreateStep>,
}

#[derive(Debug, Deserialize)]
pub struct CreateIngredient {
    pub product_id: Option<String>,
    pub raw_name_ar: Option<String>,
    pub raw_name_en: Option<String>,
    pub quantity: f64,
    pub unit: String,
    pub optional: Option<bool>,
    pub note: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateStep {
    pub text_ar: String,
    pub text_en: Option<String>,
    pub timer_minutes: Option<i64>,
}

/// How many recipes to inspect before applying an in-memory diet filter.
/// Filtering is done after loading so ingredient names can participate; the
/// caller's limit/offset are applied afterwards to keep pagination correct.
const DIET_CANDIDATE_LIMIT: i64 = 2000;

pub async fn list_recipes(
    pool: &SqlitePool,
    household_id: &str,
    meal_type: Option<&str>,
    search: Option<&str>,
    diet: Option<&str>,
    limit: i64,
    offset: i64,
) -> Result<Vec<Recipe>, CommonError> {
    let mut sql = String::from(
        "SELECT * FROM recipes WHERE is_active = 1 AND (household_id = ? OR household_id IS NULL)",
    );
    if meal_type.is_some() {
        sql.push_str(" AND meal_types LIKE ?");
    }
    // Search across both names, the description, ingredient raw names and tags
    // so a user can find a meal by what is in it, not only by its title.
    let needle = search.map(str::trim).filter(|s| !s.is_empty());
    if needle.is_some() {
        sql.push_str(
            " AND (name_ar LIKE ? OR name_en LIKE ? OR COALESCE(description, '') LIKE ? \
             OR EXISTS (SELECT 1 FROM recipe_ingredients ri WHERE ri.recipe_id = recipes.id \
             AND (ri.raw_name_ar LIKE ? OR ri.raw_name_en LIKE ?)) \
             OR EXISTS (SELECT 1 FROM recipe_tags rt WHERE rt.recipe_id = recipes.id \
             AND rt.tag LIKE ?))",
        );
    }
    sql.push_str(" ORDER BY is_system DESC, name_ar LIMIT ? OFFSET ?");
    let filtering = diet
        .filter(|d| !d.is_empty() && *d != "all")
        .and_then(crate::diet_filter::keywords)
        .is_some();
    // With a diet filter we load a wider candidate window, then paginate.
    let (fetch_limit, fetch_offset) = if filtering {
        (DIET_CANDIDATE_LIMIT, 0)
    } else {
        (limit, offset)
    };
    let mut q = sqlx::query(&sql);
    q = q.bind(household_id);
    if let Some(mt) = meal_type {
        q = q.bind(format!("%\"{mt}\"%"));
    }
    if let Some(s) = needle {
        let like = format!("%{s}%");
        for _ in 0..6 {
            q = q.bind(like.clone());
        }
    }
    let rows = q
        .bind(fetch_limit)
        .bind(fetch_offset)
        .fetch_all(pool)
        .await
        .map_err(map_db)?;
    let mut out: Vec<Recipe> = rows.into_iter().map(Recipe::from).collect();
    attach_tags(pool, &mut out).await?;

    if filtering {
        // One query for every candidate's ingredient names (raw names first,
        // catalog names as a fallback when a recipe only stores product ids).
        let names = sqlx::query(
            "SELECT ri.recipe_id, ri.raw_name_ar, ri.raw_name_en, p.name_ar AS p_ar, p.name_en AS p_en
               FROM recipe_ingredients ri
               JOIN recipes r ON r.id = ri.recipe_id
               LEFT JOIN products p ON p.id = ri.product_id
              WHERE r.is_active = 1 AND (r.household_id = ? OR r.household_id IS NULL)",
        )
        .bind(household_id)
        .fetch_all(pool)
        .await
        .map_err(map_db)?;
        let mut by_recipe: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        for row in names {
            let id: String = row.get("recipe_id");
            let text = format!(
                "{} {} {} {}",
                row.get::<Option<String>, _>("raw_name_ar").unwrap_or_default(),
                row.get::<Option<String>, _>("raw_name_en").unwrap_or_default(),
                row.get::<Option<String>, _>("p_ar").unwrap_or_default(),
                row.get::<Option<String>, _>("p_en").unwrap_or_default(),
            );
            by_recipe.entry(id).or_default().push_str(&text);
        }
        out.retain(|r| {
            crate::diet_filter::recipe_matches(diet, r, by_recipe.get(&r.id).map(String::as_str).unwrap_or(""))
        });
        out = out
            .into_iter()
            .skip(offset.max(0) as usize)
            .take(limit.max(0) as usize)
            .collect();
    }

    let mut kcal = crate::nutrition::recipe_kcal_map(pool).await?;
    let mut macros = crate::nutrition::recipe_macro_map(pool).await?;
    for r in &mut out {
        r.kcal_per_serving = kcal.remove(&r.id);
        r.macros_per_serving = macros.remove(&r.id);
    }
    Ok(out)
}

pub async fn attach_tags(pool: &SqlitePool, recipes: &mut [Recipe]) -> Result<(), CommonError> {
    for r in recipes.iter_mut() {
        r.tags = sqlx::query("SELECT tag FROM recipe_tags WHERE recipe_id = ? ORDER BY tag")
            .bind(&r.id)
            .fetch_all(pool)
            .await
            .map_err(map_db)?
            .iter()
            .map(|row| row.get::<String, _>("tag"))
            .collect();
    }
    Ok(())
}

pub async fn get_recipe(pool: &SqlitePool, id: &str) -> Result<(Recipe, Vec<RecipeIngredient>, Vec<RecipeStep>), CommonError> {
    let row = sqlx::query("SELECT * FROM recipes WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(map_db)?
        .ok_or_else(|| CommonError::NotFound("recipe".into()))?;
    let mut recipe = Recipe::from(row);
    attach_tags(pool, std::slice::from_mut(&mut recipe)).await?;
    recipe.kcal_per_serving = crate::nutrition::recipe_kcal_map(pool)
        .await?
        .get(&recipe.id)
        .copied();
    recipe.macros_per_serving = crate::nutrition::recipe_macro_map(pool)
        .await?
        .remove(&recipe.id);
    let ing_rows = sqlx::query(
        "SELECT ri.*, p.kcal_per_100g, p.protein_per_100g, p.carbs_per_100g, p.fat_per_100g,
                p.fiber_per_100g, p.grams_per_unit, p.default_unit, p.package_size, p.package_unit
         FROM recipe_ingredients ri
         LEFT JOIN products p ON p.id = ri.product_id
         WHERE ri.recipe_id = ? ORDER BY ri.sort",
    )
    .bind(id)
    .fetch_all(pool)
    .await
    .map_err(map_db)?;
    let mut ingredients = Vec::new();
    for row in ing_rows {
        let quantity: f64 = row.get("quantity");
        let unit: String = row.get("unit");
        let kcal = crate::nutrition::kcal_from_row(&row, quantity, &unit);
        let macros = crate::nutrition::macro_from_row(&row, quantity, &unit);
        let mut ing = RecipeIngredient::from(row);
        ing.kcal = kcal;
        ing.macros = macros;
        ingredients.push(ing);
    }
    let steps = sqlx::query("SELECT * FROM recipe_steps WHERE recipe_id = ? ORDER BY step_number")
        .bind(id)
        .fetch_all(pool)
        .await
        .map_err(map_db)?
        .into_iter()
        .map(RecipeStep::from)
        .collect();
    Ok((recipe, ingredients, steps))
}

pub async fn create_recipe(
    pool: &SqlitePool,
    household_id: Option<&str>,
    user_id: Option<&str>,
    input: &CreateRecipe,
) -> Result<String, CommonError> {
    if input.name_ar.trim().is_empty() {
        return Err(CommonError::Validation("recipe Arabic name is required".into()));
    }
    if input.ingredients.is_empty() {
        return Err(CommonError::Validation("recipe needs at least one ingredient".into()));
    }
    if input.steps.is_empty() {
        return Err(CommonError::Validation("recipe needs at least one step".into()));
    }
    let id = new_id();
    let now = now_rfc3339();
    sqlx::query(
        "INSERT INTO recipes (id, household_id, name_ar, name_en, description, cuisine, meal_types, difficulty, prep_minutes, cook_minutes,
            default_servings, cooking_method, equipment, allergens, storage_notes, source_type, source_ref, is_active, is_system, created_by, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 1, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(household_id)
    .bind(input.name_ar.trim())
    .bind(&input.name_en)
    .bind(&input.description)
    .bind(input.cuisine.clone().unwrap_or_else(|| "egyptian".into()))
    .bind(serde_json::to_string(&input.meal_types).unwrap())
    .bind(input.difficulty.clone().unwrap_or_else(|| "easy".into()))
    .bind(input.prep_minutes.unwrap_or(10))
    .bind(input.cook_minutes.unwrap_or(20))
    .bind(input.default_servings.unwrap_or(4).max(1))
    .bind(&input.cooking_method)
    .bind(input.equipment.as_ref().map(|e| serde_json::to_string(e).unwrap()))
    .bind(input.allergens.as_ref().map(|e| serde_json::to_string(e).unwrap()))
    .bind(&input.storage_notes)
    .bind(input.source_type.clone().unwrap_or_else(|| "user".into()))
    .bind(&input.source_ref)
    // Recipes with no household are the shared system library; per-household
    // recipes are user content. This keeps the seed guard and delete rules honest.
    .bind(if household_id.is_none() { 1i64 } else { 0i64 })
    .bind(user_id)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await
    .map_err(map_db)?;
    for (i, ing) in input.ingredients.iter().enumerate() {
        sqlx::query(
            "INSERT INTO recipe_ingredients (id, recipe_id, product_id, raw_name_ar, raw_name_en, quantity, unit, optional, note, sort)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(new_id())
        .bind(&id)
        .bind(&ing.product_id)
        .bind(&ing.raw_name_ar)
        .bind(&ing.raw_name_en)
        .bind(ing.quantity.max(0.0))
        .bind(&ing.unit)
        .bind(ing.optional.unwrap_or(false) as i64)
        .bind(&ing.note)
        .bind(i as i64)
        .execute(pool)
        .await
        .map_err(map_db)?;
    }
    for (i, st) in input.steps.iter().enumerate() {
        sqlx::query(
            "INSERT INTO recipe_steps (id, recipe_id, step_number, text_ar, text_en, timer_minutes) VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(new_id())
        .bind(&id)
        .bind((i + 1) as i64)
        .bind(&st.text_ar)
        .bind(&st.text_en)
        .bind(st.timer_minutes)
        .execute(pool)
        .await
        .map_err(map_db)?;
    }
    if let Some(tags) = &input.tags {
        for t in tags {
            let _ = sqlx::query("INSERT OR IGNORE INTO recipe_tags (id, recipe_id, tag) VALUES (?, ?, ?)")
                .bind(new_id())
                .bind(&id)
                .bind(t.trim())
                .execute(pool)
                .await;
        }
    }
    Ok(id)
}

pub async fn delete_recipe(pool: &SqlitePool, id: &str) -> Result<(), CommonError> {
    sqlx::query("UPDATE recipes SET is_active = 0 WHERE id = ? AND is_system = 0")
        .bind(id)
        .execute(pool)
        .await
        .map_err(map_db)?;
    Ok(())
}

// ---------- Batch preparation guides (التدبير والطبخ الاقتصادي) ----------

#[derive(Debug, Serialize)]
pub struct BatchGuide {
    pub id: String,
    pub title_ar: String,
    pub title_en: Option<String>,
    pub category: Option<String>,
    pub batch_size_text: Option<String>,
    pub ingredients: serde_json::Value,
    pub steps: serde_json::Value,
    pub portions: Option<i64>,
    pub storage: serde_json::Value,
    pub safety_source: Option<String>,
    pub safety_source_date: Option<String>,
}

impl From<sqlx::sqlite::SqliteRow> for BatchGuide {
    fn from(r: sqlx::sqlite::SqliteRow) -> Self {
        BatchGuide {
            id: r.get("id"),
            title_ar: r.get("title_ar"),
            title_en: r.get("title_en"),
            category: r.get("category"),
            batch_size_text: r.get("batch_size_text"),
            ingredients: r
                .get::<Option<String>, _>("ingredients")
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or(serde_json::Value::Null),
            steps: r
                .get::<Option<String>, _>("steps")
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or(serde_json::Value::Null),
            portions: r.get("portions"),
            storage: r
                .get::<Option<String>, _>("storage")
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or(serde_json::Value::Null),
            safety_source: r.get("safety_source"),
            safety_source_date: r.get("safety_source_date"),
        }
    }
}

pub async fn list_batch_guides(pool: &SqlitePool, household_id: &str) -> Result<Vec<BatchGuide>, CommonError> {
    let rows = sqlx::query(
        "SELECT * FROM batch_guides WHERE is_system = 1 OR household_id = ? ORDER BY category, title_ar",
    )
    .bind(household_id)
    .fetch_all(pool)
    .await
    .map_err(map_db)?;
    Ok(rows.into_iter().map(BatchGuide::from).collect())
}

#[derive(Debug, Deserialize)]
pub struct ConvertGuideRequest {
    pub product_id: String,
    pub quantity: f64,
    pub unit: String,
    pub location_id: Option<String>,
    pub expiry_date: Option<String>,
}

/// Converts a batch-preparation result into an inventory lot.
pub async fn convert_guide_to_inventory(
    pool: &SqlitePool,
    household_id: &str,
    guide_id: &str,
    req: &ConvertGuideRequest,
) -> Result<crate::inventory::StockLot, CommonError> {
    sqlx::query("SELECT id FROM batch_guides WHERE id = ?")
        .bind(guide_id)
        .fetch_optional(pool)
        .await
        .map_err(map_db)?
        .ok_or_else(|| CommonError::NotFound("batch guide".into()))?;
    crate::inventory::add_lot(
        pool,
        household_id,
        &crate::inventory::CreateLot {
            product_id: req.product_id.clone(),
            quantity: req.quantity,
            unit: req.unit.clone(),
            location_id: req.location_id.clone(),
            purchase_date: Some(crate::today().format("%Y-%m-%d").to_string()),
            expiry_date: req.expiry_date.clone(),
            estimated_cost_minor: None,
            notes: Some("batch preparation".into()),
        },
        "batch",
    )
    .await
}

