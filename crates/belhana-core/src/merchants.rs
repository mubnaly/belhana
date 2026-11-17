//! Merchants and their locations.

use belhana_common::CommonError;
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};

use crate::{map_db, new_id, now_rfc3339};

#[derive(Debug, Serialize)]
pub struct Merchant {
    pub id: String,
    pub name: String,
    pub slug: Option<String>,
    pub mtype: String,
    pub verification_status: String,
    pub phone: Option<String>,
    pub facebook_url: Option<String>,
    pub website_url: Option<String>,
    pub notes: Option<String>,
    pub locations: Vec<MerchantLocation>,
}

impl From<sqlx::sqlite::SqliteRow> for Merchant {
    fn from(r: sqlx::sqlite::SqliteRow) -> Self {
        Merchant {
            id: r.get("id"),
            name: r.get("name"),
            slug: r.get("slug"),
            mtype: r.get("type"),
            verification_status: r.get("verification_status"),
            phone: r.get("phone"),
            facebook_url: r.get("facebook_url"),
            website_url: r.get("website_url"),
            notes: r.get("notes"),
            locations: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct MerchantLocation {
    pub id: String,
    pub merchant_id: String,
    pub city: Option<String>,
    pub area: Option<String>,
    pub address: Option<String>,
    pub is_confirmed: bool,
    pub notes: Option<String>,
}

impl From<sqlx::sqlite::SqliteRow> for MerchantLocation {
    fn from(r: sqlx::sqlite::SqliteRow) -> Self {
        MerchantLocation {
            id: r.get("id"),
            merchant_id: r.get("merchant_id"),
            city: r.get("city"),
            area: r.get("area"),
            address: r.get("address"),
            is_confirmed: r.get::<i64, _>("is_confirmed") != 0,
            notes: r.get("notes"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateMerchant {
    pub name: String,
    pub slug: Option<String>,
    pub mtype: Option<String>,
    pub phone: Option<String>,
    pub facebook_url: Option<String>,
    pub website_url: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateLocation {
    pub city: Option<String>,
    pub area: Option<String>,
    pub address: Option<String>,
    pub is_confirmed: Option<bool>,
    pub notes: Option<String>,
}

async fn attach_locations(pool: &SqlitePool, m: &mut Merchant) -> Result<(), CommonError> {
    let rows = sqlx::query("SELECT * FROM merchant_locations WHERE merchant_id = ?")
        .bind(&m.id)
        .fetch_all(pool)
        .await
        .map_err(map_db)?;
    m.locations = rows.into_iter().map(MerchantLocation::from).collect();
    Ok(())
}

pub async fn list_merchants(pool: &SqlitePool, search: Option<&str>) -> Result<Vec<Merchant>, CommonError> {
    let sql = match search {
        Some(_) => "SELECT * FROM merchants WHERE name LIKE ? ORDER BY name",
        None => "SELECT * FROM merchants ORDER BY name",
    };
    let mut q = sqlx::query(sql);
    if let Some(s) = search {
        q = q.bind(format!("%{s}%"));
    }
    let rows = q.fetch_all(pool).await.map_err(map_db)?;
    let mut out = Vec::new();
    for r in rows {
        let mut m = Merchant::from(r);
        attach_locations(pool, &mut m).await?;
        out.push(m);
    }
    Ok(out)
}

pub async fn get_merchant(pool: &SqlitePool, id: &str) -> Result<Merchant, CommonError> {
    let row = sqlx::query("SELECT * FROM merchants WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(map_db)?
        .ok_or_else(|| CommonError::NotFound("merchant".into()))?;
    let mut m = Merchant::from(row);
    attach_locations(pool, &mut m).await?;
    Ok(m)
}

pub async fn find_by_slug(pool: &SqlitePool, slug: &str) -> Result<Option<Merchant>, CommonError> {
    if let Some(r) = sqlx::query("SELECT * FROM merchants WHERE slug = ?")
        .bind(slug)
        .fetch_optional(pool)
        .await
        .map_err(map_db)?
    {
        let mut m = Merchant::from(r);
        attach_locations(pool, &mut m).await?;
        return Ok(Some(m));
    }
    Ok(None)
}

pub async fn create_merchant(pool: &SqlitePool, input: &CreateMerchant) -> Result<Merchant, CommonError> {
    if input.name.trim().is_empty() {
        return Err(CommonError::Validation("merchant name is required".into()));
    }
    let id = new_id();
    sqlx::query(
        "INSERT INTO merchants (id, name, slug, type, verification_status, phone, facebook_url, website_url, notes, created_at)
         VALUES (?, ?, ?, ?, 'unverified', ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(input.name.trim())
    .bind(&input.slug)
    .bind(input.mtype.clone().unwrap_or_else(|| "grocery".into()))
    .bind(&input.phone)
    .bind(&input.facebook_url)
    .bind(&input.website_url)
    .bind(&input.notes)
    .bind(now_rfc3339())
    .execute(pool)
    .await
    .map_err(map_db)?;
    get_merchant(pool, &id).await
}

/// Merchant location must be confirmed by the user/admin; never inferred.
pub async fn add_location(
    pool: &SqlitePool,
    merchant_id: &str,
    input: &CreateLocation,
) -> Result<MerchantLocation, CommonError> {
    get_merchant(pool, merchant_id).await?;
    let id = new_id();
    sqlx::query(
        "INSERT INTO merchant_locations (id, merchant_id, city, area, address, is_confirmed, notes) VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(merchant_id)
    .bind(&input.city)
    .bind(&input.area)
    .bind(&input.address)
    .bind(input.is_confirmed.unwrap_or(false) as i64)
    .bind(&input.notes)
    .execute(pool)
    .await
    .map_err(map_db)?;
    let row = sqlx::query("SELECT * FROM merchant_locations WHERE id = ?")
        .bind(&id)
        .fetch_one(pool)
        .await
        .map_err(map_db)?;
    Ok(MerchantLocation::from(row))
}
