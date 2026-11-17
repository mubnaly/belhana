//! Shopping lists generated from accepted plans + current stock, plus the
//! purchase workflow (trips, actual prices, inventory updates after
//! confirmation only).

use belhana_common::CommonError;
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};

use crate::{catalog, costing, households::Household, inventory, map_db, new_id, now_rfc3339, planning, pricing, today};
use belhana_common::units::universal_convert_str;

#[derive(Debug, Serialize)]
pub struct ShoppingList {
    pub id: String,
    pub household_id: String,
    pub plan_id: Option<String>,
    pub status: String,
    pub trip_date: Option<String>,
    pub policy: String,
    pub estimated_total_minor: i64,
    pub created_at: String,
    pub completed_at: Option<String>,
    pub items: Vec<ShoppingItem>,
}

#[derive(Debug, Serialize)]
pub struct ShoppingItem {
    pub id: String,
    pub product_id: String,
    pub product_name_ar: String,
    pub required_qty: f64,
    pub required_unit: String,
    pub stock_qty: f64,
    pub shopping_qty: f64,
    pub shopping_unit: String,
    pub package_size: Option<f64>,
    pub package_unit: Option<String>,
    pub estimated_unit_price_minor: Option<i64>,
    pub estimated_total_minor: Option<i64>,
    pub price_observation_id: Option<String>,
    pub merchant_id: Option<String>,
    pub price_freshness: Option<String>,
    /// Set for "buy this much money's worth" lines (spices such as salt).
    pub buy_by_amount_minor: Option<i64>,
    pub purchased_qty: Option<f64>,
    pub actual_price_minor: Option<i64>,
    pub purchased: bool,
    pub status: String,
    pub notes: Option<String>,
}

impl From<sqlx::sqlite::SqliteRow> for ShoppingItem {
    fn from(r: sqlx::sqlite::SqliteRow) -> Self {
        ShoppingItem {
            id: r.get("id"),
            product_id: r.get("product_id"),
            product_name_ar: r.get("product_name_ar"),
            required_qty: r.get("required_qty"),
            required_unit: r.get("required_unit"),
            stock_qty: r.get::<Option<f64>, _>("stock_qty").unwrap_or(0.0),
            shopping_qty: r.get::<Option<f64>, _>("shopping_qty").unwrap_or(0.0),
            shopping_unit: r.get::<Option<String>, _>("shopping_unit").unwrap_or_default(),
            package_size: r.get("package_size"),
            package_unit: r.get("package_unit"),
            estimated_unit_price_minor: r.get("estimated_unit_price_minor"),
            estimated_total_minor: r.get("estimated_total_minor"),
            price_observation_id: r.get("price_observation_id"),
            merchant_id: r.get("merchant_id"),
            price_freshness: r.get("price_freshness"),
            buy_by_amount_minor: r.get("buy_by_amount_minor"),
            purchased_qty: r.get("purchased_qty"),
            actual_price_minor: r.get("actual_price_minor"),
            purchased: r.get::<i64, _>("purchased") != 0,
            status: r.get("status"),
            notes: r.get("notes"),
        }
    }
}

fn list_from_row(r: sqlx::sqlite::SqliteRow) -> ShoppingList {
    ShoppingList {
        id: r.get("id"),
        household_id: r.get("household_id"),
        plan_id: r.get("plan_id"),
        status: r.get("status"),
        trip_date: r.get("trip_date"),
        policy: r.get("policy"),
        estimated_total_minor: r.get::<Option<i64>, _>("estimated_total_minor").unwrap_or(0),
        created_at: r.get("created_at"),
        completed_at: r.get("completed_at"),
        items: Vec::new(),
    }
}

/// Generates (or regenerates) a shopping list from a plan and current stock.
/// Inventory is NOT deducted here — only proposed quantities are computed.
pub async fn generate_from_plan(
    pool: &SqlitePool,
    household_id: &str,
    plan_id: &str,
) -> Result<ShoppingList, CommonError> {
    let hh = crate::households::get_household(pool, household_id).await?;
    let plan = planning::read_plan(pool, plan_id).await?;

    // Aggregate required quantities per (product, unit) across the plan's slots.
    let mut required: std::collections::HashMap<String, (f64, String)> = Default::default();
    for slot in &plan.slots {
        let ings = costing::ingredient_quantities(pool, &slot.recipe_id, slot.servings).await?;
        for (pid, qty, unit) in ings {
            let e = required.entry(pid.clone()).or_insert((0.0, unit.clone()));
            if e.1 == unit {
                e.0 += qty;
            } else {
                // try conversion, else keep the first unit and sum converted
                if let Some(conv) = universal_convert_str(qty, &unit, &e.1) {
                    e.0 += conv;
                } else {
                    // incompatible unit → separate entry keyed by unit
                    let key = format!("{pid}|{unit}");
                    let e2 = required.entry(key.clone()).or_insert((qty, unit.clone()));
                    let _ = e2;
                }
            }
        }
    }

    // Create a new list (replacing any previous draft for this plan).
    sqlx::query("UPDATE shopping_lists SET status = 'completed', completed_at = ? WHERE plan_id = ? AND status IN ('draft','active')")
        .bind(now_rfc3339())
        .bind(plan_id)
        .execute(pool)
        .await
        .map_err(map_db)?;
    let list_id = new_id();
    sqlx::query(
        "INSERT INTO shopping_lists (id, household_id, plan_id, status, trip_date, policy, created_at) VALUES (?, ?, ?, 'active', ?, ?, ?)",
    )
    .bind(&list_id)
    .bind(household_id)
    .bind(plan_id)
    .bind(today().format("%Y-%m-%d").to_string())
    .bind(&hh.price_policy)
    .bind(now_rfc3339())
    .execute(pool)
    .await
    .map_err(map_db)?;

    let mut estimated_total = 0i64;
    let mut sort = 0;
    for (pid, (qty, unit)) in &required {
        if *qty <= 0.0 {
            continue;
        }
        let product = catalog::get_product(pool, pid).await?;
        let stock = inventory::product_stock(pool, household_id, pid, unit).await?;
        let mut shopping_qty = (qty - stock).max(0.0);
        // Package rounding where a package size is known and unit matches.
        let (pkg_size, pkg_unit) = (product.package_size, product.package_unit.clone());
        if let Some(ps) = pkg_size {
            if pkg_unit.as_deref() == Some(unit.as_str()) && shopping_qty > 0.0 {
                let packs = (shopping_qty / ps).ceil();
                shopping_qty = packs * ps;
            }
        }
        let (unit_price, obs_id, merchant, freshness) =
            match pricing::best_observation_for_product(pool, pid, &hh.price_policy).await? {
                Some(o) => match costing::price_per_unit(&o, unit) {
                    Some(per) => (
                        Some(per.round() as i64),
                        Some(o.id.clone()),
                        o.merchant_id.clone(),
                        Some(pricing::freshness(o.observation_date.as_deref(), o.review_status == "verified").to_string()),
                    ),
                    None => (None, None, None, Some("unit_mismatch".to_string())),
                },
                None => (None, None, None, Some("none".to_string())),
            };
        let total = unit_price.map(|p| (p as f64 * shopping_qty).round() as i64);
        if let Some(t) = total {
            estimated_total += t;
        }
        sqlx::query(
            "INSERT INTO shopping_list_items (id, list_id, product_id, required_qty, required_unit, stock_qty, shopping_qty, shopping_unit,
                package_size, package_unit, estimated_unit_price_minor, estimated_total_minor, price_observation_id, merchant_id, price_freshness, status, sort)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'pending', ?)",
        )
        .bind(new_id())
        .bind(&list_id)
        .bind(pid)
        .bind(*qty)
        .bind(unit)
        .bind(stock)
        .bind(shopping_qty)
        .bind(unit)
        .bind(pkg_size)
        .bind(&pkg_unit)
        .bind(unit_price)
        .bind(total)
        .bind(&obs_id)
        .bind(&merchant)
        .bind(&freshness)
        .bind(sort)
        .execute(pool)
        .await
        .map_err(map_db)?;
        sort += 1;
    }
    // Fixed-spend lines: spices are bought by money ("20 EGP of salt"), not by
    // a recipe quantity. Only add a line when the product is not already on the
    // list from the plan, so we never duplicate a purchase.
    for (pid, amount) in crate::rules::fixed_spends(pool, household_id).await? {
        let already_listed = required
            .keys()
            .any(|k| k == &pid || k.starts_with(&format!("{pid}|")));
        if already_listed {
            continue;
        }
        if catalog::get_product(pool, &pid).await.is_err() {
            continue;
        }
        estimated_total += amount;
        sqlx::query(
            "INSERT INTO shopping_list_items
                (id, list_id, product_id, required_qty, required_unit, stock_qty, shopping_qty, shopping_unit,
                 estimated_unit_price_minor, estimated_total_minor, price_freshness, buy_by_amount_minor, notes, status, sort)
             VALUES (?, ?, ?, 1, 'spend', 0, 1, 'spend', ?, ?, 'none', ?, ?, 'pending', ?)",
        )
        .bind(new_id())
        .bind(&list_id)
        .bind(&pid)
        .bind(amount)
        .bind(amount)
        .bind(amount)
        .bind(Some("fixed spend rule — buy this amount of money"))
        .bind(sort)
        .execute(pool)
        .await
        .map_err(map_db)?;
        sort += 1;
    }

    sqlx::query("UPDATE shopping_lists SET estimated_total_minor = ? WHERE id = ?")
        .bind(estimated_total)
        .bind(&list_id)
        .execute(pool)
        .await
        .map_err(map_db)?;
    get_list(pool, household_id, &list_id).await
}

pub async fn get_list(pool: &SqlitePool, household_id: &str, id: &str) -> Result<ShoppingList, CommonError> {
    let row = sqlx::query("SELECT * FROM shopping_lists WHERE id = ? AND household_id = ?")
        .bind(id)
        .bind(household_id)
        .fetch_optional(pool)
        .await
        .map_err(map_db)?
        .ok_or_else(|| CommonError::NotFound("shopping list".into()))?;
    let mut list = list_from_row(row);
    let items = sqlx::query(
        "SELECT i.*, p.name_ar AS product_name_ar FROM shopping_list_items i
         JOIN products p ON p.id = i.product_id
         WHERE i.list_id = ? AND i.status != 'removed' ORDER BY i.sort",
    )
    .bind(&list.id)
    .fetch_all(pool)
    .await
    .map_err(map_db)?;
    list.items = items.into_iter().map(ShoppingItem::from).collect();
    Ok(list)
}

pub async fn active_list(pool: &SqlitePool, household_id: &str) -> Result<Option<ShoppingList>, CommonError> {
    let row = sqlx::query(
        "SELECT * FROM shopping_lists WHERE household_id = ? AND status IN ('draft','active') ORDER BY created_at DESC LIMIT 1",
    )
    .bind(household_id)
    .fetch_optional(pool)
    .await
    .map_err(map_db)?;
    match row {
        Some(r) => Ok(Some(get_list(pool, household_id, &r.get::<String, _>("id")).await?)),
        None => Ok(None),
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateItemRequest {
    pub purchased_qty: Option<f64>,
    pub actual_price_minor: Option<i64>,
    pub purchased: Option<bool>,
    pub substitute_product_id: Option<String>,
    pub shopping_qty: Option<f64>,
    pub notes: Option<String>,
}

/// Marks purchase progress on one item. Inventory is updated only when the
/// whole trip is confirmed (complete_trip) — or per-item with confirm.
pub async fn update_item(
    pool: &SqlitePool,
    household_id: &str,
    list_id: &str,
    item_id: &str,
    req: &UpdateItemRequest,
) -> Result<ShoppingList, CommonError> {
    let _ = get_list(pool, household_id, &list_id).await?;
    let item = sqlx::query("SELECT * FROM shopping_list_items WHERE id = ? AND list_id = ?")
        .bind(item_id)
        .bind(list_id)
        .fetch_optional(pool)
        .await
        .map_err(map_db)?
        .ok_or_else(|| CommonError::NotFound("shopping item".into()))?;
    let purchased = req.purchased.unwrap_or_else(|| item.get::<i64, _>("purchased") != 0);
    sqlx::query(
        "UPDATE shopping_list_items SET purchased_qty = COALESCE(?, purchased_qty),
            actual_price_minor = COALESCE(?, actual_price_minor),
            purchased = ?, substitute_product_id = COALESCE(?, substitute_product_id),
            shopping_qty = COALESCE(?, shopping_qty), notes = COALESCE(?, notes),
            status = CASE WHEN ? THEN 'purchased' ELSE status END
         WHERE id = ?",
    )
    .bind(req.purchased_qty)
    .bind(req.actual_price_minor)
    .bind(purchased as i64)
    .bind(&req.substitute_product_id)
    .bind(req.shopping_qty)
    .bind(&req.notes)
    .bind(purchased)
    .bind(item_id)
    .execute(pool)
    .await
    .map_err(map_db)?;
    get_list(pool, household_id, &list_id).await
}

/// Confirms a completed trip: records the trip, actual expenses, and adds
/// purchased quantities to inventory (the ONLY place stock grows from shopping).
pub async fn complete_trip(
    pool: &SqlitePool,
    household_id: &str,
    list_id: &str,
    merchant_id: Option<&str>,
    actor: &str,
) -> Result<ShoppingList, CommonError> {
    let list = get_list(pool, household_id, &list_id).await?;
    let hh = crate::households::get_household(pool, household_id).await?;
    let mut actual_total = 0i64;
    let mut planned_total = 0i64;
    let trip_id = new_id();
    let date = today().format("%Y-%m-%d").to_string();
    sqlx::query(
        "INSERT INTO shopping_trips (id, household_id, list_id, date, merchant_id, created_at) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&trip_id)
    .bind(household_id)
    .bind(&list_id)
    .bind(&date)
    .bind(merchant_id)
    .bind(now_rfc3339())
    .execute(pool)
    .await
    .map_err(map_db)?;

    for item in &list.items {
        if item.status == "removed" {
            continue;
        }
        planned_total += item.estimated_total_minor.unwrap_or(0);
        if !item.purchased {
            continue;
        }
        let qty = item.purchased_qty.unwrap_or(item.shopping_qty);
        let price = item.actual_price_minor.or(item.estimated_total_minor);
        if let Some(p) = price {
            actual_total += p;
        }
        // purchase item record
        sqlx::query(
            "INSERT INTO purchase_items (id, trip_id, product_id, quantity, unit, actual_price_minor, price_observation_id)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(new_id())
        .bind(&trip_id)
        .bind(&item.product_id)
        .bind(qty)
        .bind(&item.shopping_unit)
        .bind(price)
        .bind(&item.price_observation_id)
        .execute(pool)
        .await
        .map_err(map_db)?;
        // A fixed-spend purchase (e.g. 20 EGP of salt) has no known quantity,
        // so it must not create a fake "1 spend" stock lot.
        if item.buy_by_amount_minor.is_none() {
            let lot = inventory::add_lot(
                pool,
                household_id,
                &inventory::CreateLot {
                    product_id: item.product_id.clone(),
                    quantity: qty,
                    unit: item.shopping_unit.clone(),
                    location_id: None,
                    purchase_date: Some(date.clone()),
                    expiry_date: None,
                    estimated_cost_minor: price,
                    notes: Some("shopping purchase".into()),
                },
                "shopping",
            )
            .await?;
            sqlx::query("UPDATE purchase_items SET lot_id = ? WHERE trip_id = ? AND product_id = ?")
                .bind(&lot.id)
                .bind(&trip_id)
                .bind(&item.product_id)
                .execute(pool)
                .await
                .map_err(map_db)?;
        }
    }

    if actual_total > 0 {
        sqlx::query(
            "INSERT INTO actual_expenses (id, household_id, category, amount_minor, date, trip_id, created_by, created_at)
             VALUES (?, ?, 'groceries', ?, ?, ?, ?, ?)",
        )
        .bind(new_id())
        .bind(household_id)
        .bind(actual_total)
        .bind(&date)
        .bind(&trip_id)
        .bind(actor)
        .bind(now_rfc3339())
        .execute(pool)
        .await
        .map_err(map_db)?;
    }
    sqlx::query("UPDATE shopping_trips SET total_planned_minor = ?, total_actual_minor = ? WHERE id = ?")
        .bind(planned_total)
        .bind(actual_total)
        .bind(&trip_id)
        .execute(pool)
        .await
        .map_err(map_db)?;
    sqlx::query("UPDATE shopping_lists SET status = 'completed', completed_at = ? WHERE id = ?")
        .bind(now_rfc3339())
        .bind(&list_id)
        .execute(pool)
        .await
        .map_err(map_db)?;
    crate::audit::log(
        pool,
        Some(household_id),
        Some(actor),
        "complete_trip",
        "shopping_list",
        &list_id,
        None,
        Some(serde_json::json!({"trip_id": trip_id, "actual_total": actual_total})),
    )
    .await?;
    let _ = hh;
    get_list(pool, household_id, &list_id).await
}

/// Public helper used by budget module.
pub async fn touch_pool(pool: &SqlitePool) -> Result<(), CommonError> {
    sqlx::query("SELECT 1").execute(pool).await.map_err(map_db)?;
    Ok(())
}

/// Household helper re-export for occasions module.
pub async fn load_household(pool: &SqlitePool, household_id: &str) -> Result<Household, CommonError> {
    crate::households::get_household(pool, household_id).await
}
