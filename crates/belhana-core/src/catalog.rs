//! Product catalog, categories, aliases, and unit definitions.

use belhana_common::CommonError;
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};

use crate::{map_db, new_id, now_rfc3339};

#[derive(Debug, Clone, Serialize)]
pub struct Category {
    pub id: String,
    pub slug: String,
    pub name_ar: String,
    pub name_en: String,
    pub sort: i64,
}

impl From<sqlx::sqlite::SqliteRow> for Category {
    fn from(r: sqlx::sqlite::SqliteRow) -> Self {
        Category {
            id: r.get("id"),
            slug: r.get("slug"),
            name_ar: r.get("name_ar"),
            name_en: r.get("name_en"),
            sort: r.get("sort"),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Product {
    pub id: String,
    pub category_id: Option<String>,
    pub name_ar: String,
    pub name_en: Option<String>,
    pub default_unit: Option<String>,
    pub comparison_unit: Option<String>,
    pub package_size: Option<f64>,
    pub package_unit: Option<String>,
    pub brand: Option<String>,
    pub is_active: bool,
    pub notes: Option<String>,
    /// Editorial estimate of calories per 100 g (or 100 ml for liquids).
    pub kcal_per_100g: Option<f64>,
    /// Approximate grams per 100 g (protein, carbs, fat, fibre).
    pub protein_per_100g: Option<f64>,
    pub carbs_per_100g: Option<f64>,
    pub fat_per_100g: Option<f64>,
    pub fiber_per_100g: Option<f64>,
    /// Typical edible grams in one default unit (piece/package/...).
    pub grams_per_unit: Option<f64>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<sqlx::sqlite::SqliteRow> for Product {
    fn from(r: sqlx::sqlite::SqliteRow) -> Self {
        Product {
            id: r.get("id"),
            category_id: r.get("category_id"),
            name_ar: r.get("name_ar"),
            name_en: r.get("name_en"),
            default_unit: r.get("default_unit"),
            comparison_unit: r.get("comparison_unit"),
            package_size: r.get("package_size"),
            package_unit: r.get("package_unit"),
            brand: r.get("brand"),
            is_active: r.get::<i64, _>("is_active") != 0,
            notes: r.get("notes"),
            kcal_per_100g: r.try_get::<Option<f64>, _>("kcal_per_100g").ok().flatten(),
            protein_per_100g: r.try_get::<Option<f64>, _>("protein_per_100g").ok().flatten(),
            carbs_per_100g: r.try_get::<Option<f64>, _>("carbs_per_100g").ok().flatten(),
            fat_per_100g: r.try_get::<Option<f64>, _>("fat_per_100g").ok().flatten(),
            fiber_per_100g: r.try_get::<Option<f64>, _>("fiber_per_100g").ok().flatten(),
            grams_per_unit: r.try_get::<Option<f64>, _>("grams_per_unit").ok().flatten(),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateProduct {
    pub category_id: Option<String>,
    pub name_ar: String,
    pub name_en: Option<String>,
    pub default_unit: Option<String>,
    pub comparison_unit: Option<String>,
    pub package_size: Option<f64>,
    pub package_unit: Option<String>,
    pub brand: Option<String>,
    pub aliases: Option<Vec<String>>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProduct {
    pub category_id: Option<String>,
    pub name_ar: Option<String>,
    pub name_en: Option<String>,
    pub default_unit: Option<String>,
    pub comparison_unit: Option<String>,
    pub package_size: Option<f64>,
    pub package_unit: Option<String>,
    pub brand: Option<String>,
    pub is_active: Option<bool>,
    pub notes: Option<String>,
}

pub async fn list_categories(pool: &SqlitePool) -> Result<Vec<Category>, CommonError> {
    let rows = sqlx::query("SELECT * FROM product_categories ORDER BY sort, name_ar")
        .fetch_all(pool)
        .await
        .map_err(map_db)?;
    Ok(rows.into_iter().map(Category::from).collect())
}

pub async fn get_category(pool: &SqlitePool, id: &str) -> Result<Category, CommonError> {
    let row = sqlx::query("SELECT * FROM product_categories WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(map_db)?
        .ok_or_else(|| CommonError::NotFound("category".into()))?;
    Ok(Category::from(row))
}

pub async fn list_products(
    pool: &SqlitePool,
    category_id: Option<&str>,
    search: Option<&str>,
    limit: i64,
    offset: i64,
) -> Result<Vec<Product>, CommonError> {
    let mut sql = String::from("SELECT * FROM products WHERE is_active = 1");
    if category_id.is_some() {
        sql.push_str(" AND category_id = ?");
    }
    if search.is_some() {
        sql.push_str(" AND (name_ar LIKE ? OR name_en LIKE ?)");
    }
    sql.push_str(" ORDER BY name_ar LIMIT ? OFFSET ?");
    let mut q = sqlx::query(&sql);
    if let Some(c) = category_id {
        q = q.bind(c);
    }
    if let Some(s) = search {
        let like = format!("%{s}%");
        q = q.bind(like.clone()).bind(like);
    }
    let rows = q.bind(limit).bind(offset).fetch_all(pool).await.map_err(map_db)?;
    Ok(rows.into_iter().map(Product::from).collect())
}

pub async fn get_product(pool: &SqlitePool, id: &str) -> Result<Product, CommonError> {
    let row = sqlx::query("SELECT * FROM products WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(map_db)?
        .ok_or_else(|| CommonError::NotFound("product".into()))?;
    Ok(Product::from(row))
}

pub async fn create_product(pool: &SqlitePool, input: &CreateProduct) -> Result<Product, CommonError> {
    if input.name_ar.trim().is_empty() {
        return Err(CommonError::Validation("product Arabic name is required".into()));
    }
    let id = new_id();
    let now = now_rfc3339();
    sqlx::query(
        "INSERT INTO products (id, category_id, name_ar, name_en, default_unit, comparison_unit, package_size, package_unit, brand, is_active, notes, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 1, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&input.category_id)
    .bind(input.name_ar.trim())
    .bind(&input.name_en)
    .bind(&input.default_unit)
    .bind(&input.comparison_unit)
    .bind(input.package_size)
    .bind(&input.package_unit)
    .bind(&input.brand)
    .bind(&input.notes)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await
    .map_err(map_db)?;
    if let Some(aliases) = &input.aliases {
        for a in aliases {
            if a.trim().is_empty() {
                continue;
            }
            let _ = sqlx::query(
                "INSERT OR IGNORE INTO product_aliases (id, product_id, alias_ar, source) VALUES (?, ?, ?, 'user')",
            )
            .bind(new_id())
            .bind(&id)
            .bind(a.trim())
            .execute(pool)
            .await;
        }
    }
    get_product(pool, &id).await
}

pub async fn update_product(
    pool: &SqlitePool,
    id: &str,
    input: &UpdateProduct,
) -> Result<Product, CommonError> {
    let cur = get_product(pool, id).await?;
    sqlx::query(
        "UPDATE products SET category_id=?, name_ar=?, name_en=?, default_unit=?, comparison_unit=?,
         package_size=?, package_unit=?, brand=?, is_active=?, notes=?, updated_at=? WHERE id=?",
    )
    .bind(input.category_id.clone().or(cur.category_id.clone()))
    .bind(input.name_ar.clone().unwrap_or(cur.name_ar.clone()))
    .bind(input.name_en.clone().or(cur.name_en.clone()))
    .bind(input.default_unit.clone().or(cur.default_unit.clone()))
    .bind(input.comparison_unit.clone().or(cur.comparison_unit.clone()))
    .bind(input.package_size.or(cur.package_size))
    .bind(input.package_unit.clone().or(cur.package_unit.clone()))
    .bind(input.brand.clone().or(cur.brand.clone()))
    .bind(input.is_active.unwrap_or(cur.is_active) as i64)
    .bind(input.notes.clone().or(cur.notes.clone()))
    .bind(now_rfc3339())
    .bind(id)
    .execute(pool)
    .await
    .map_err(map_db)?;
    get_product(pool, id).await
}

/// Finds a product by Arabic name or alias (used by import matching).
pub async fn find_by_arabic_name(pool: &SqlitePool, name: &str) -> Result<Option<Product>, CommonError> {
    // exact name first
    if let Some(p) = sqlx::query("SELECT * FROM products WHERE name_ar = ? AND is_active = 1 LIMIT 1")
        .bind(name.trim())
        .fetch_optional(pool)
        .await
        .map_err(map_db)?
    {
        return Ok(Some(Product::from(p)));
    }
    // then alias
    if let Some(p) = sqlx::query(
        "SELECT p.* FROM products p JOIN product_aliases a ON a.product_id = p.id
         WHERE a.alias_ar = ? AND p.is_active = 1 LIMIT 1",
    )
    .bind(name.trim())
    .fetch_optional(pool)
    .await
    .map_err(map_db)?
    {
        return Ok(Some(Product::from(p)));
    }
    Ok(None)
}

/// Products referenced by a recipe ingredient ingredient-lookup by Arabic name.
pub async fn products_by_ids(pool: &SqlitePool, ids: &[String]) -> Result<Vec<Product>, CommonError> {
    if ids.is_empty() {
        return Ok(vec![]);
    }
    let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let sql = format!("SELECT * FROM products WHERE id IN ({placeholders})");
    let mut q = sqlx::query(&sql);
    for id in ids {
        q = q.bind(id);
    }
    let rows = q.fetch_all(pool).await.map_err(map_db)?;
    Ok(rows.into_iter().map(Product::from).collect())
}
