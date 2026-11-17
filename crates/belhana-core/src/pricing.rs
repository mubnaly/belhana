//! Price observations: dated offers, never permanent product attributes.
//!
//! Every price display must carry its observation date, source and freshness.
//! Ambiguous units are never silently resolved; baseline user estimates are a
//! separate status (`estimate`) and never masquerade as merchant quotes.

use belhana_common::CommonError;
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

use crate::{map_db, new_id, now_rfc3339};

#[derive(Debug, Clone, Serialize)]
pub struct PriceObservation {
    pub id: String,
    pub product_id: Option<String>,
    pub product_variant: Option<String>,
    pub merchant_id: Option<String>,
    pub location_id: Option<String>,
    pub amount_minor: Option<i64>,
    pub currency: String,
    pub unit: Option<String>,
    pub unit_status: String,
    pub package_size: Option<f64>,
    pub package_unit: Option<String>,
    pub observation_date: Option<String>,
    pub submitted_at: String,
    pub source_type: String,
    pub source_url: Option<String>,
    pub raw_text: Option<String>,
    pub raw_product_label: Option<String>,
    pub confidence: Option<f64>,
    pub review_status: String,
    pub previous_amount_minor: Option<i64>,
    pub promotion_note: Option<String>,
    pub reviewed_by: Option<String>,
    pub reviewed_at: Option<String>,
    pub batch_id: Option<String>,
    pub baseline_owner_user_id: Option<String>,
    pub occurrence_index: i64,
    pub notes: Option<String>,
}

impl From<sqlx::sqlite::SqliteRow> for PriceObservation {
    fn from(r: sqlx::sqlite::SqliteRow) -> Self {
        PriceObservation {
            id: r.get("id"),
            product_id: r.get("product_id"),
            product_variant: r.get("product_variant"),
            merchant_id: r.get("merchant_id"),
            location_id: r.get("location_id"),
            amount_minor: r.get("amount_minor"),
            currency: r.get("currency"),
            unit: r.get("unit"),
            unit_status: r.get("unit_status"),
            package_size: r.get("package_size"),
            package_unit: r.get("package_unit"),
            observation_date: r.get("observation_date"),
            submitted_at: r.get("submitted_at"),
            source_type: r.get("source_type"),
            source_url: r.get("source_url"),
            raw_text: r.get("raw_text"),
            raw_product_label: r.get("raw_product_label"),
            confidence: r.get("confidence"),
            review_status: r.get("review_status"),
            previous_amount_minor: r.get("previous_amount_minor"),
            promotion_note: r.get("promotion_note"),
            reviewed_by: r.get("reviewed_by"),
            reviewed_at: r.get("reviewed_at"),
            batch_id: r.get("batch_id"),
            baseline_owner_user_id: r.get("baseline_owner_user_id"),
            occurrence_index: r.get("occurrence_index"),
            notes: r.get("notes"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreatePriceObservation {
    pub product_id: String,
    pub product_variant: Option<String>,
    pub merchant_id: Option<String>,
    pub location_id: Option<String>,
    pub amount_minor: i64,
    pub unit: Option<String>,
    pub unit_status: Option<String>,
    pub observation_date: Option<String>,
    pub source_type: Option<String>,
    pub source_url: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReviewAction {
    pub action: String, // verify | reject | confirm_unit | confirm_date | map_product | mark_stale
    pub unit: Option<String>,
    pub observation_date: Option<String>,
    pub product_id: Option<String>,
    pub note: Option<String>,
}

/// Freshness classification for display and engine use.
pub fn freshness(observation_date: Option<&str>, verified: bool) -> &'static str {
    if !verified {
        return "unverified";
    }
    let Some(date) = observation_date.and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
    else {
        return "unknown";
    };
    let age = (chrono::Utc::now().date_naive() - date).num_days();
    if age < 0 {
        "future"
    } else if age <= 7 {
        "fresh"
    } else if age <= 30 {
        "aging"
    } else {
        "stale"
    }
}

pub async fn list_observations(
    pool: &SqlitePool,
    product_id: Option<&str>,
    merchant_id: Option<&str>,
    review_status: Option<&str>,
    batch_id: Option<&str>,
    limit: i64,
    offset: i64,
) -> Result<Vec<PriceObservation>, CommonError> {
    let mut sql = String::from("SELECT * FROM price_observations WHERE 1=1");
    if product_id.is_some() {
        sql.push_str(" AND product_id = ?");
    }
    if merchant_id.is_some() {
        sql.push_str(" AND merchant_id = ?");
    }
    if review_status.is_some() {
        sql.push_str(" AND review_status = ?");
    }
    if batch_id.is_some() {
        sql.push_str(" AND batch_id = ?");
    }
    sql.push_str(" ORDER BY submitted_at DESC, occurrence_index LIMIT ? OFFSET ?");
    let mut q = sqlx::query(&sql);
    if let Some(v) = product_id {
        q = q.bind(v);
    }
    if let Some(v) = merchant_id {
        q = q.bind(v);
    }
    if let Some(v) = review_status {
        q = q.bind(v);
    }
    if let Some(v) = batch_id {
        q = q.bind(v);
    }
    let rows = q.bind(limit).bind(offset).fetch_all(pool).await.map_err(map_db)?;
    Ok(rows.into_iter().map(PriceObservation::from).collect())
}

pub async fn get_observation(pool: &SqlitePool, id: &str) -> Result<PriceObservation, CommonError> {
    let row = sqlx::query("SELECT * FROM price_observations WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(map_db)?
        .ok_or_else(|| CommonError::NotFound("price observation".into()))?;
    Ok(PriceObservation::from(row))
}

/// Manual price entry — first-class workflow.
pub async fn create_observation(
    pool: &SqlitePool,
    actor: Option<&str>,
    input: &CreatePriceObservation,
) -> Result<PriceObservation, CommonError> {
    if input.amount_minor <= 0 {
        return Err(CommonError::Validation("amount must be positive".into()));
    }
    let unit_status = input.unit_status.clone().unwrap_or_else(|| {
        if input.unit.is_some() {
            "confirmed".into()
        } else {
            "unresolved".into()
        }
    });
    // A price without a unit is not safely comparable: keep it flagged.
    let review_status = if unit_status == "unresolved" {
        "needs_unit_confirmation"
    } else {
        "unreviewed"
    };
    let id = new_id();
    sqlx::query(
        "INSERT INTO price_observations (id, product_id, product_variant, merchant_id, location_id, amount_minor, currency, unit, unit_status,
            observation_date, submitted_at, source_type, source_url, raw_text, review_status, occurrence_index, notes)
         VALUES (?, ?, ?, ?, ?, ?, 'EGP', ?, ?, ?, ?, ?, ?, ?, ?, 0, ?)",
    )
    .bind(&id)
    .bind(&input.product_id)
    .bind(&input.product_variant)
    .bind(&input.merchant_id)
    .bind(&input.location_id)
    .bind(input.amount_minor)
    .bind(&input.unit)
    .bind(&unit_status)
    .bind(&input.observation_date)
    .bind(now_rfc3339())
    .bind(input.source_type.clone().unwrap_or_else(|| "manual".into()))
    .bind(&input.source_url)
    .bind(&input.notes)
    .bind(review_status)
    .bind(&input.notes)
    .execute(pool)
    .await
    .map_err(map_db)?;
    record_event(pool, &id, "created", actor, None, review_status, Some("manual entry")).await?;
    get_observation(pool, &id).await
}

/// User-supplied editable baseline estimate (separate from merchant quotes).
pub async fn create_baseline_estimate(
    pool: &SqlitePool,
    user_id: &str,
    product_id: &str,
    amount_minor: i64,
    unit: &str,
    observation_date: Option<String>,
) -> Result<PriceObservation, CommonError> {
    let id = new_id();
    sqlx::query(
        "INSERT INTO price_observations (id, product_id, merchant_id, amount_minor, currency, unit, unit_status,
            observation_date, submitted_at, source_type, review_status, baseline_owner_user_id, notes)
         VALUES (?, ?, NULL, ?, 'EGP', ?, 'confirmed', ?, ?, 'user_baseline', 'estimate', ?, 'User-supplied editable baseline estimate')",
    )
    .bind(&id)
    .bind(product_id)
    .bind(amount_minor)
    .bind(unit)
    .bind(observation_date.clone().or_else(|| Some(crate::today().format("%Y-%m-%d").to_string())))
    .bind(now_rfc3339())
    .bind(user_id)
    .execute(pool)
    .await
    .map_err(map_db)?;
    record_event(pool, &id, "created", Some(user_id), None, "estimate", Some("baseline estimate")).await?;
    get_observation(pool, &id).await
}

/// Applies a human review action; every correction is audited.
pub async fn review_observation(
    pool: &SqlitePool,
    actor: &str,
    observation_id: &str,
    action: &ReviewAction,
) -> Result<PriceObservation, CommonError> {
    let cur = get_observation(pool, observation_id).await?;
    let (new_status, new_unit, new_date, new_product) = match action.action.as_str() {
        "verify" => (Some("verified".to_string()), None, None, None),
        "reject" => (Some("rejected".to_string()), None, None, None),
        "confirm_unit" => {
            let unit = action
                .unit
                .clone()
                .ok_or_else(|| CommonError::Validation("unit is required to confirm".into()))?;
            (Some("unreviewed".to_string()), Some(unit), None, None)
        }
        "confirm_date" => {
            let date = action
                .observation_date
                .clone()
                .ok_or_else(|| CommonError::Validation("date is required to confirm".into()))?;
            chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d")
                .map_err(|_| CommonError::Validation("invalid date format, expected YYYY-MM-DD".into()))?;
            (Some("unreviewed".to_string()), None, Some(date), None)
        }
        "map_product" => {
            let pid = action
                .product_id
                .clone()
                .ok_or_else(|| CommonError::Validation("product_id is required".into()))?;
            (Some("unreviewed".to_string()), None, None, Some(pid))
        }
        "mark_stale" => (Some("stale".to_string()), None, None, None),
        other => return Err(CommonError::Validation(format!("unknown review action: {other}"))),
    };

    let status = new_status.clone().unwrap_or_else(|| cur.review_status.clone());
    sqlx::query(
        "UPDATE price_observations SET review_status = ?,
            unit = COALESCE(?, unit),
            unit_status = CASE WHEN ? IS NOT NULL THEN 'confirmed' ELSE unit_status END,
            observation_date = COALESCE(?, observation_date),
            product_id = COALESCE(?, product_id),
            reviewed_by = ?, reviewed_at = ?
         WHERE id = ?",
    )
    .bind(&status)
    .bind(&action.unit)
    .bind(&action.unit)
    .bind(&action.observation_date)
    .bind(&action.product_id)
    .bind(actor)
    .bind(now_rfc3339())
    .bind(observation_id)
    .execute(pool)
    .await
    .map_err(map_db)?;
    record_event(pool, observation_id, &action.action, Some(actor), Some(&cur.review_status), &status, action.note.as_deref()).await?;
    get_observation(pool, observation_id).await
}

pub async fn record_event(
    pool: &SqlitePool,
    observation_id: &str,
    action: &str,
    actor: Option<&str>,
    from_status: Option<&str>,
    to_status: &str,
    note: Option<&str>,
) -> Result<(), CommonError> {
    sqlx::query(
        "INSERT INTO price_review_events (id, observation_id, action, actor_user_id, from_status, to_status, note, created_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(Uuid::new_v4().to_string())
    .bind(observation_id)
    .bind(action)
    .bind(actor)
    .bind(from_status)
    .bind(to_status)
    .bind(note)
    .bind(now_rfc3339())
    .execute(pool)
    .await
    .map_err(map_db)?;
    Ok(())
}

/// Picks the best usable price observation for a product according to policy.
/// `freshest_verified`: newest verified observation with a resolved unit.
/// `cheapest_verified`: lowest current verified price per unit.
pub async fn best_observation_for_product(
    pool: &SqlitePool,
    product_id: &str,
    policy: &str,
) -> Result<Option<PriceObservation>, CommonError> {
    let order = match policy {
        "cheapest_verified" => "amount_minor ASC, observation_date DESC",
        _ => "observation_date DESC, submitted_at DESC",
    };
    let sql = format!(
        "SELECT * FROM price_observations
         WHERE product_id = ? AND review_status IN ('verified','estimate')
           AND amount_minor IS NOT NULL AND unit IS NOT NULL
         ORDER BY {order} LIMIT 1"
    );
    let row = sqlx::query(&sql).bind(product_id).fetch_optional(pool).await.map_err(map_db)?;
    Ok(row.map(PriceObservation::from))
}

/// All usable observations for a product (verified or estimate, priced, with unit).
pub async fn usable_observations(
    pool: &SqlitePool,
    product_id: &str,
) -> Result<Vec<PriceObservation>, CommonError> {
    let rows = sqlx::query(
        "SELECT * FROM price_observations
         WHERE product_id = ? AND review_status IN ('verified','estimate')
           AND amount_minor IS NOT NULL AND unit IS NOT NULL
         ORDER BY observation_date DESC",
    )
    .bind(product_id)
    .fetch_all(pool)
    .await
    .map_err(map_db)?;
    Ok(rows.into_iter().map(PriceObservation::from).collect())
}

/// Marks observations older than `days` as stale (maintenance worker).
pub async fn sweep_stale(pool: &SqlitePool, days: i64) -> Result<u64, CommonError> {
    let cutoff = (chrono::Utc::now() - chrono::Duration::days(days))
        .date_naive()
        .format("%Y-%m-%d")
        .to_string();
    let res = sqlx::query(
        "UPDATE price_observations SET review_status = 'stale'
         WHERE review_status = 'verified' AND observation_date IS NOT NULL AND observation_date < ?",
    )
    .bind(&cutoff)
    .execute(pool)
    .await
    .map_err(map_db)?;
    Ok(res.rows_affected())
}
