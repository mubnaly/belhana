//! Inventory: pantry / refrigerator / freezer stock with audited movements.
//!
//! Deduction policy: inventory is *never* deducted merely because a plan
//! exists. Stock updates happen on confirmed purchases, explicit consumption,
//! audited adjustments, or marked-prepared meals per the household workflow.

use belhana_common::CommonError;
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};

use crate::{map_db, new_id, now_rfc3339};

#[derive(Debug, Clone, Serialize)]
pub struct StorageLocation {
    pub id: String,
    pub household_id: String,
    pub name_ar: String,
    pub name_en: Option<String>,
    pub kind: String,
}

impl From<sqlx::sqlite::SqliteRow> for StorageLocation {
    fn from(r: sqlx::sqlite::SqliteRow) -> Self {
        StorageLocation {
            id: r.get("id"),
            household_id: r.get("household_id"),
            name_ar: r.get("name_ar"),
            name_en: r.get("name_en"),
            kind: r.get("kind"),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct StockLot {
    pub id: String,
    pub household_id: String,
    pub product_id: String,
    pub quantity: f64,
    pub unit: String,
    pub location_id: Option<String>,
    pub purchase_date: Option<String>,
    pub opened_date: Option<String>,
    pub expiry_date: Option<String>,
    pub source: String,
    pub estimated_cost_minor: Option<i64>,
    pub notes: Option<String>,
    pub status: String,
    /// Unused stock is kept for the following week by default.
    pub carry_over: bool,
    pub created_at: String,
    pub product_name_ar: Option<String>,
    pub location_name: Option<String>,
    /// Estimated calories for the quantity currently in stock.
    pub estimated_kcal: Option<i64>,
}

fn lot_from_row_with(r: sqlx::sqlite::SqliteRow, with_names: bool) -> StockLot {
    let quantity: f64 = r.get("quantity");
    let unit: String = r.get("unit");
    let estimated_kcal = if with_names {
        crate::nutrition::kcal_from_row(&r, quantity, &unit)
    } else {
        None
    };
    let mut lot = StockLot {
        id: r.get("id"),
        household_id: r.get("household_id"),
        product_id: r.get("product_id"),
        quantity,
        unit,
        location_id: r.get("location_id"),
        purchase_date: r.get("purchase_date"),
        opened_date: r.get("opened_date"),
        expiry_date: r.get("expiry_date"),
        source: r.get("source"),
        estimated_cost_minor: r.get("estimated_cost_minor"),
        notes: r.get("notes"),
        status: r.get("status"),
        carry_over: r.get::<i64, _>("carry_over") != 0,
        created_at: r.get("created_at"),
        product_name_ar: None,
        location_name: None,
        estimated_kcal,
    };
    if with_names {
        lot.product_name_ar = r.try_get("product_name_ar").ok();
        lot.location_name = r.try_get("location_name").ok();
    }
    lot
}

#[derive(Debug, Deserialize)]
pub struct CreateLot {
    pub product_id: String,
    pub quantity: f64,
    pub unit: String,
    pub location_id: Option<String>,
    pub purchase_date: Option<String>,
    pub expiry_date: Option<String>,
    pub estimated_cost_minor: Option<i64>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateLot {
    pub quantity: Option<f64>,
    pub unit: Option<String>,
    pub location_id: Option<String>,
    pub opened_date: Option<String>,
    pub expiry_date: Option<String>,
    pub notes: Option<String>,
}

pub async fn ensure_default_locations(pool: &SqlitePool, household_id: &str) -> Result<(), CommonError> {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM storage_locations WHERE household_id = ?")
        .bind(household_id)
        .fetch_one(pool)
        .await
        .map_err(map_db)?;
    if count > 0 {
        return Ok(());
    }
    for (name_ar, name_en, kind) in [
        ("المخزن", "Pantry", "pantry"),
        ("الثلاجة", "Refrigerator", "fridge"),
        ("الفريزر", "Freezer", "freezer"),
    ] {
        sqlx::query(
            "INSERT INTO storage_locations (id, household_id, name_ar, name_en, kind, created_at) VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(new_id())
        .bind(household_id)
        .bind(name_ar)
        .bind(name_en)
        .bind(kind)
        .bind(now_rfc3339())
        .execute(pool)
        .await
        .map_err(map_db)?;
    }
    Ok(())
}

pub async fn list_locations(pool: &SqlitePool, household_id: &str) -> Result<Vec<StorageLocation>, CommonError> {
    let rows = sqlx::query("SELECT * FROM storage_locations WHERE household_id = ? ORDER BY created_at")
        .bind(household_id)
        .fetch_all(pool)
        .await
        .map_err(map_db)?;
    Ok(rows.into_iter().map(StorageLocation::from).collect())
}

pub async fn list_lots(
    pool: &SqlitePool,
    household_id: &str,
    location_id: Option<&str>,
    kind: Option<&str>,
    status: Option<&str>,
) -> Result<Vec<StockLot>, CommonError> {
    let mut sql = String::from(
        "SELECT l.*, p.name_ar AS product_name_ar, s.name_ar AS location_name,
                p.kcal_per_100g, p.grams_per_unit, p.default_unit
         FROM stock_lots l
         JOIN products p ON p.id = l.product_id
         LEFT JOIN storage_locations s ON s.id = l.location_id
         WHERE l.household_id = ?",
    );
    if location_id.is_some() {
        sql.push_str(" AND l.location_id = ?");
    }
    if kind.is_some() {
        sql.push_str(" AND s.kind = ?");
    }
    // Carry-over lots first, then FEFO within each group.
    sql.push_str(" AND l.status = COALESCE(?, 'active') ORDER BY l.carry_over DESC, l.expiry_date IS NULL, l.expiry_date, l.created_at");
    let mut q = sqlx::query(&sql);
    q = q.bind(household_id);
    if let Some(v) = location_id {
        q = q.bind(v);
    }
    if let Some(v) = kind {
        q = q.bind(v);
    }
    q = q.bind(status);
    let rows = q.fetch_all(pool).await.map_err(map_db)?;
    Ok(rows.into_iter().map(|r| lot_from_row_with(r, true)).collect())
}

pub async fn get_lot(pool: &SqlitePool, household_id: &str, id: &str) -> Result<StockLot, CommonError> {
    let row = sqlx::query(
        "SELECT l.*, p.name_ar AS product_name_ar, s.name_ar AS location_name,
                p.kcal_per_100g, p.grams_per_unit, p.default_unit
         FROM stock_lots l JOIN products p ON p.id = l.product_id
         LEFT JOIN storage_locations s ON s.id = l.location_id
         WHERE l.id = ? AND l.household_id = ?",
    )
    .bind(id)
    .bind(household_id)
    .fetch_optional(pool)
    .await
    .map_err(map_db)?
    .ok_or_else(|| CommonError::NotFound("stock lot".into()))?;
    Ok(lot_from_row_with(row, true))
}

pub async fn add_lot(
    pool: &SqlitePool,
    household_id: &str,
    input: &CreateLot,
    source: &str,
) -> Result<StockLot, CommonError> {
    if input.quantity <= 0.0 {
        return Err(CommonError::Validation("quantity must be positive".into()));
    }
    let id = new_id();
    let now = now_rfc3339();
    sqlx::query(
        "INSERT INTO stock_lots (id, household_id, product_id, quantity, unit, location_id, purchase_date, expiry_date, source, estimated_cost_minor, notes, status, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'active', ?, ?)",
    )
    .bind(&id)
    .bind(household_id)
    .bind(&input.product_id)
    .bind(input.quantity)
    .bind(&input.unit)
    .bind(&input.location_id)
    .bind(&input.purchase_date)
    .bind(&input.expiry_date)
    .bind(source)
    .bind(input.estimated_cost_minor)
    .bind(&input.notes)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await
    .map_err(map_db)?;
    sqlx::query(
        "INSERT INTO inventory_movements (id, household_id, lot_id, movement_type, quantity, unit, to_location_id, reason, created_at)
         VALUES (?, ?, ?, 'add', ?, ?, ?, ?, ?)",
    )
    .bind(new_id())
    .bind(household_id)
    .bind(&id)
    .bind(input.quantity)
    .bind(&input.unit)
    .bind(&input.location_id)
    .bind(source)
    .bind(&now)
    .execute(pool)
    .await
    .map_err(map_db)?;
    get_lot(pool, household_id, &id).await
}

/// Consumes quantity FEFO (first-expiring-first-out) across lots of a product.
/// Returns the lots actually consumed.
pub async fn consume_fefo(
    pool: &SqlitePool,
    household_id: &str,
    product_id: &str,
    mut qty: f64,
    unit: &str,
    reason: &str,
    related_meal: Option<&str>,
) -> Result<Vec<(String, f64)>, CommonError> {
    let lots = sqlx::query(
        "SELECT * FROM stock_lots WHERE household_id = ? AND product_id = ? AND status = 'active' AND quantity > 0
         ORDER BY carry_over DESC, expiry_date IS NULL, expiry_date, created_at",
    )
    .bind(household_id)
    .bind(product_id)
    .fetch_all(pool)
    .await
    .map_err(map_db)?;
    let mut consumed = Vec::new();
    for r in lots {
        if qty <= 0.0 {
            break;
        }
        let lot: StockLot = lot_from_row_with(r, false);
        // Only deduct when the unit matches directly (cross-unit conversion is
        // handled by the planner before calling this).
        if lot.unit != unit {
            continue;
        }
        let take = lot.quantity.min(qty);
        let remaining = lot.quantity - take;
        let status = if remaining <= 0.0001 { "consumed" } else { "active" };
        sqlx::query("UPDATE stock_lots SET quantity = ?, status = ?, updated_at = ? WHERE id = ?")
            .bind(remaining)
            .bind(status)
            .bind(now_rfc3339())
            .bind(&lot.id)
            .execute(pool)
            .await
            .map_err(map_db)?;
        sqlx::query(
            "INSERT INTO inventory_movements (id, household_id, lot_id, movement_type, quantity, unit, reason, related_planned_meal_id, created_at)
             VALUES (?, ?, ?, 'consume', ?, ?, ?, ?, ?)",
        )
        .bind(new_id())
        .bind(household_id)
        .bind(&lot.id)
        .bind(take)
        .bind(unit)
        .bind(reason)
        .bind(related_meal)
        .bind(now_rfc3339())
        .execute(pool)
        .await
        .map_err(map_db)?;
        consumed.push((lot.id, take));
        qty -= take;
    }
    Ok(consumed)
}

/// Audited correction: sets quantity and records an adjustment movement.
pub async fn adjust_lot(
    pool: &SqlitePool,
    household_id: &str,
    lot_id: &str,
    new_qty: f64,
    reason: &str,
) -> Result<StockLot, CommonError> {
    let lot = get_lot(pool, household_id, lot_id).await?;
    if new_qty < 0.0 {
        return Err(CommonError::Validation("quantity cannot be negative".into()));
    }
    sqlx::query("UPDATE stock_lots SET quantity = ?, updated_at = ? WHERE id = ?")
        .bind(new_qty)
        .bind(now_rfc3339())
        .bind(lot_id)
        .execute(pool)
        .await
        .map_err(map_db)?;
    let delta = new_qty - lot.quantity;
    let mtype = if delta >= 0.0 { "adjust" } else { "discard" };
    sqlx::query(
        "INSERT INTO inventory_movements (id, household_id, lot_id, movement_type, quantity, unit, reason, created_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(new_id())
    .bind(household_id)
    .bind(lot_id)
    .bind(mtype)
    .bind(delta.abs())
    .bind(&lot.unit)
    .bind(reason)
    .bind(now_rfc3339())
    .execute(pool)
    .await
    .map_err(map_db)?;
    get_lot(pool, household_id, lot_id).await
}

pub async fn move_lot(
    pool: &SqlitePool,
    household_id: &str,
    lot_id: &str,
    to_location: &str,
) -> Result<StockLot, CommonError> {
    let lot = get_lot(pool, household_id, lot_id).await?;
    sqlx::query("UPDATE stock_lots SET location_id = ?, updated_at = ? WHERE id = ?")
        .bind(to_location)
        .bind(now_rfc3339())
        .bind(lot_id)
        .execute(pool)
        .await
        .map_err(map_db)?;
    sqlx::query(
        "INSERT INTO inventory_movements (id, household_id, lot_id, movement_type, quantity, unit, from_location_id, to_location_id, created_at)
         VALUES (?, ?, ?, 'move', ?, ?, ?, ?, ?)",
    )
    .bind(new_id())
    .bind(household_id)
    .bind(lot_id)
    .bind(lot.quantity)
    .bind(&lot.unit)
    .bind(&lot.location_id)
    .bind(to_location)
    .bind(now_rfc3339())
    .execute(pool)
    .await
    .map_err(map_db)?;
    get_lot(pool, household_id, lot_id).await
}

pub async fn update_lot(
    pool: &SqlitePool,
    household_id: &str,
    lot_id: &str,
    input: &UpdateLot,
) -> Result<StockLot, CommonError> {
    let lot = get_lot(pool, household_id, lot_id).await?;
    sqlx::query(
        "UPDATE stock_lots SET unit = ?, opened_date = ?, expiry_date = ?, notes = ?, updated_at = ? WHERE id = ?",
    )
    .bind(input.unit.clone().unwrap_or(lot.unit.clone()))
    .bind(input.opened_date.clone().or(lot.opened_date.clone()))
    .bind(input.expiry_date.clone().or(lot.expiry_date.clone()))
    .bind(input.notes.clone().or(lot.notes.clone()))
    .bind(now_rfc3339())
    .bind(lot_id)
    .execute(pool)
    .await
    .map_err(map_db)?;
    if let Some(q) = input.quantity {
        if (q - lot.quantity).abs() > f64::EPSILON {
            adjust_lot(pool, household_id, lot_id, q, "edited").await?;
        }
    }
    if let Some(loc) = &input.location_id {
        if Some(loc.as_str()) != lot.location_id.as_deref() {
            move_lot(pool, household_id, lot_id, loc).await?;
        }
    }
    get_lot(pool, household_id, lot_id).await
}

pub async fn record_waste(
    pool: &SqlitePool,
    household_id: &str,
    product_id: &str,
    qty: f64,
    unit: &str,
    reason: Option<&str>,
    estimated_cost_minor: Option<i64>,
) -> Result<(), CommonError> {
    let date = crate::today().format("%Y-%m-%d").to_string();
    sqlx::query(
        "INSERT INTO food_waste_records (id, household_id, product_id, quantity, unit, date, reason, estimated_cost_minor, created_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(new_id())
    .bind(household_id)
    .bind(product_id)
    .bind(qty)
    .bind(unit)
    .bind(&date)
    .bind(reason)
    .bind(estimated_cost_minor)
    .bind(now_rfc3339())
    .execute(pool)
    .await
    .map_err(map_db)?;
    // mark matching lots consumed
    let _ = consume_fefo(pool, household_id, product_id, qty, unit, "waste", None).await;
    Ok(())
}

/// Active lots that remain available for next week's plan. Unsigned stock is
/// never thrown away at week boundaries; the planner and shopping list treat
/// these quantities as already owned.
pub async fn list_leftovers(
    pool: &SqlitePool,
    household_id: &str,
) -> Result<Vec<StockLot>, CommonError> {
    let rows = sqlx::query(
        "SELECT l.*, p.name_ar AS product_name_ar, s.name_ar AS location_name,
                p.kcal_per_100g, p.grams_per_unit, p.default_unit
         FROM stock_lots l JOIN products p ON p.id = l.product_id
         LEFT JOIN storage_locations s ON s.id = l.location_id
         WHERE l.household_id = ? AND l.status = 'active' AND l.quantity > 0 AND l.carry_over = 1
         ORDER BY l.expiry_date IS NULL, l.expiry_date, l.created_at",
    )
    .bind(household_id)
    .fetch_all(pool)
    .await
    .map_err(map_db)?;
    Ok(rows.into_iter().map(|r| lot_from_row_with(r, true)).collect())
}

/// Opts a lot in or out of next-week carry-over.
pub async fn set_carry_over(
    pool: &SqlitePool,
    household_id: &str,
    lot_id: &str,
    carry_over: bool,
) -> Result<StockLot, CommonError> {
    get_lot(pool, household_id, lot_id).await?;
    sqlx::query("UPDATE stock_lots SET carry_over = ?, updated_at = ? WHERE id = ?")
        .bind(carry_over as i64)
        .bind(now_rfc3339())
        .bind(lot_id)
        .execute(pool)
        .await
        .map_err(map_db)?;
    get_lot(pool, household_id, lot_id).await
}

/// Lots expiring within `days` — for reminders and dashboard alerts.
pub async fn expiring_soon(pool: &SqlitePool, household_id: &str, days: i64) -> Result<Vec<StockLot>, CommonError> {
    let until = (crate::today() + chrono::Duration::days(days))
        .format("%Y-%m-%d")
        .to_string();
    let rows = sqlx::query(
        "SELECT l.*, p.name_ar AS product_name_ar, s.name_ar AS location_name,
                p.kcal_per_100g, p.grams_per_unit, p.default_unit
         FROM stock_lots l JOIN products p ON p.id = l.product_id
         LEFT JOIN storage_locations s ON s.id = l.location_id
         WHERE l.household_id = ? AND l.status = 'active' AND l.quantity > 0
           AND l.expiry_date IS NOT NULL AND l.expiry_date <= ?
         ORDER BY l.expiry_date",
    )
    .bind(household_id)
    .bind(&until)
    .fetch_all(pool)
    .await
    .map_err(map_db)?;
    Ok(rows.into_iter().map(|r| lot_from_row_with(r, true)).collect())
}

/// Estimated inventory valuation (explicitly an estimate).
pub async fn valuation(pool: &SqlitePool, household_id: &str) -> Result<i64, CommonError> {
    let v: i64 = sqlx::query_scalar(
        "SELECT CAST(COALESCE(SUM(quantity * COALESCE(estimated_cost_minor, 0)), 0) AS INTEGER)
         FROM stock_lots WHERE household_id = ? AND status = 'active'",
    )
    .bind(household_id)
    .fetch_one(pool)
    .await
    .map_err(map_db)?;
    Ok(v)
}

/// Total available stock of a product (active lots).
pub async fn product_stock(
    pool: &SqlitePool,
    household_id: &str,
    product_id: &str,
    unit: &str,
) -> Result<f64, CommonError> {
    let rows = sqlx::query(
        "SELECT quantity, unit FROM stock_lots WHERE household_id = ? AND product_id = ? AND status = 'active' AND quantity > 0",
    )
    .bind(household_id)
    .bind(product_id)
    .fetch_all(pool)
    .await
    .map_err(map_db)?;
    let mut total = 0.0;
    for r in rows {
        let lot_unit: String = r.get("unit");
        if lot_unit == unit {
            total += r.get::<f64, _>("quantity");
        }
    }
    Ok(total)
}
