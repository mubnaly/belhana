//! Price import batches: paste → parse → propose → human review → approve.

use belhana_common::CommonError;
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};

use crate::{catalog, import_parse, map_db, merchants, new_id, now_rfc3339, pricing};

#[derive(Debug, Serialize)]
pub struct SourcePost {
    pub id: String,
    pub merchant_id: Option<String>,
    pub raw_text: String,
    pub source_type: String,
    pub source_url: Option<String>,
    pub original_date_text: Option<String>,
    pub normalized_date: Option<String>,
    pub date_status: String,
    pub weekday_mismatch: bool,
    pub location_status: String,
    pub captured_at: String,
}

impl From<sqlx::sqlite::SqliteRow> for SourcePost {
    fn from(r: sqlx::sqlite::SqliteRow) -> Self {
        SourcePost {
            id: r.get("id"),
            merchant_id: r.get("merchant_id"),
            raw_text: r.get("raw_text"),
            source_type: r.get("source_type"),
            source_url: r.get("source_url"),
            original_date_text: r.get("original_date_text"),
            normalized_date: r.get("normalized_date"),
            date_status: r.get("date_status"),
            weekday_mismatch: r.get::<i64, _>("weekday_mismatch") != 0,
            location_status: r.get("location_status"),
            captured_at: r.get("captured_at"),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ImportBatch {
    pub id: String,
    pub source_post_id: String,
    pub merchant_id: Option<String>,
    pub batch_status: String,
    pub created_at: String,
    pub reviewed_at: Option<String>,
    pub notes: Option<String>,
    pub line_count: i64,
}

impl From<sqlx::sqlite::SqliteRow> for ImportBatch {
    fn from(r: sqlx::sqlite::SqliteRow) -> Self {
        ImportBatch {
            id: r.get("id"),
            source_post_id: r.get("source_post_id"),
            merchant_id: r.get("merchant_id"),
            batch_status: r.get("batch_status"),
            created_at: r.get("created_at"),
            reviewed_at: r.get("reviewed_at"),
            notes: r.get("notes"),
            line_count: 0,
        }
    }
}

/// Creates an import batch from pasted text. Every extracted observation starts
/// unreviewed / needs-confirmation; nothing becomes verified here.
pub async fn create_batch_from_text(
    pool: &SqlitePool,
    actor: &str,
    raw_text: &str,
    source_type: &str,
    source_url: Option<&str>,
    merchant_id: Option<&str>,
) -> Result<(ImportBatch, Vec<pricing::PriceObservation>), CommonError> {
    if raw_text.trim().is_empty() {
        return Err(CommonError::Validation("source text is empty".into()));
    }
    let parsed = import_parse::parse_post(raw_text);

    // Preserve the source post verbatim.
    let post_id = new_id();
    sqlx::query(
        "INSERT INTO price_source_posts (id, merchant_id, raw_text, source_type, source_url, original_date_text, normalized_date, date_status, weekday_mismatch, location_status, captured_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 'unknown', ?)",
    )
    .bind(&post_id)
    .bind(merchant_id)
    .bind(&parsed.original_text)
    .bind(source_type)
    .bind(source_url)
    .bind(&parsed.original_date_text)
    .bind(&parsed.proposed_date)
    .bind(
        if parsed.weekday_mismatch == Some(true) || parsed.proposed_date.is_some() {
            "needs_date_confirmation"
        } else {
            "unreviewed"
        },
    )
    .bind(parsed.weekday_mismatch.map(|m| m as i64).unwrap_or(0))
    .bind(now_rfc3339())
    .execute(pool)
    .await
    .map_err(map_db)?;

    let batch_id = new_id();
    sqlx::query(
        "INSERT INTO price_import_batches (id, source_post_id, merchant_id, batch_status, created_by, created_at)
         VALUES (?, ?, ?, 'imported_needs_review', ?, ?)",
    )
    .bind(&batch_id)
    .bind(&post_id)
    .bind(merchant_id)
    .bind(actor)
    .bind(now_rfc3339())
    .execute(pool)
    .await
    .map_err(map_db)?;

    let mut created = Vec::new();
    for (i, line) in parsed.lines.iter().enumerate() {
        // Product mapping proposal (best-effort; human confirms).
        let product_id = match catalog::find_by_arabic_name(pool, &line.label).await? {
            Some(p) => Some(p.id),
            None => fuzzy_match_product(pool, &line.label).await?,
        };
        let price = line.prices_minor.first().copied();
        let review_status = if product_id.is_none() {
            "needs_product_mapping"
        } else if line.proposed_unit.is_none() {
            "needs_unit_confirmation"
        } else {
            "unreviewed"
        };
        let id = new_id();
        sqlx::query(
            "INSERT INTO price_observations (id, product_id, merchant_id, amount_minor, currency, unit, unit_status, package_size, package_unit,
                observation_date, submitted_at, source_type, source_url, raw_text, raw_product_label, review_status, previous_amount_minor,
                batch_id, occurrence_index, notes)
             VALUES (?, ?, ?, ?, 'EGP', ?, 'unresolved', ?, ?, NULL, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(&product_id)
        .bind(merchant_id)
        .bind(price)
        .bind(&line.proposed_unit)
        .bind(line.proposed_qty)
        .bind(line.proposed_unit.as_deref())
        .bind(now_rfc3339())
        .bind(source_type)
        .bind(source_url)
        .bind(&line.raw)
        .bind(&line.label)
        .bind(review_status)
        .bind(line.previous_price_minor)
        .bind(&batch_id)
        .bind(i as i64)
        .bind(line.flags.join(", "))
        .execute(pool)
        .await
        .map_err(map_db)?;
        pricing::record_event(pool, &id, "created", Some(actor), None, review_status, Some("import batch")).await?;
        created.push(pricing::get_observation(pool, &id).await?);
    }

    let mut batch = get_batch(pool, &batch_id).await?;
    batch.line_count = created.len() as i64;
    Ok((batch, created))
}

/// Substring-based fuzzy product match with confidence score.
pub async fn fuzzy_match_product(
    pool: &SqlitePool,
    label: &str,
) -> Result<Option<String>, CommonError> {
    let rows = sqlx::query(
        "SELECT p.id, p.name_ar FROM products p
         WHERE p.is_active = 1 AND ? LIKE '%' || p.name_ar || '%'
         LIMIT 1",
    )
    .bind(label)
    .fetch_optional(pool)
    .await
    .map_err(map_db)?;
    if let Some(r) = rows {
        let id: String = r.get("id");
        return Ok(Some(id));
    }
    // reverse: label is a substring of a product name/alias
    let rows = sqlx::query(
        "SELECT p.id FROM products p LEFT JOIN product_aliases a ON a.product_id = p.id
         WHERE p.is_active = 1 AND (p.name_ar LIKE '%' || ? || '%' OR a.alias_ar LIKE '%' || ? || '%')
         LIMIT 1",
    )
    .bind(label)
    .bind(label)
    .fetch_optional(pool)
    .await
    .map_err(map_db)?;
    Ok(rows.map(|r| r.get::<String, _>("id")))
}

pub async fn get_batch(pool: &SqlitePool, id: &str) -> Result<ImportBatch, CommonError> {
    let row = sqlx::query("SELECT * FROM price_import_batches WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(map_db)?
        .ok_or_else(|| CommonError::NotFound("import batch".into()))?;
    Ok(ImportBatch::from(row))
}

pub async fn list_batches(pool: &SqlitePool) -> Result<Vec<ImportBatch>, CommonError> {
    let rows = sqlx::query("SELECT * FROM price_import_batches ORDER BY created_at DESC")
        .fetch_all(pool)
        .await
        .map_err(map_db)?;
    let mut out = Vec::new();
    for r in rows {
        let mut b = ImportBatch::from(r);
        b.line_count = sqlx::query_scalar("SELECT COUNT(*) FROM price_observations WHERE batch_id = ?")
            .bind(&b.id)
            .fetch_one(pool)
            .await
            .map_err(map_db)?;
        out.push(b);
    }
    Ok(out)
}

pub async fn get_source_post(pool: &SqlitePool, id: &str) -> Result<SourcePost, CommonError> {
    let row = sqlx::query("SELECT * FROM price_source_posts WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(map_db)?
        .ok_or_else(|| CommonError::NotFound("source post".into()))?;
    Ok(SourcePost::from(row))
}

#[derive(Debug, Deserialize)]
pub struct ConfirmDateRequest {
    pub observation_date: Option<String>, // YYYY-MM-DD
    pub reject_date: Option<bool>,
}

/// Confirms or rejects the post's date for a batch (human decision).
pub async fn confirm_batch_date(
    pool: &SqlitePool,
    actor: &str,
    batch_id: &str,
    req: &ConfirmDateRequest,
) -> Result<(), CommonError> {
    let batch = get_batch(pool, batch_id).await?;
    let status = if req.reject_date == Some(true) {
        "rejected"
    } else {
        "confirmed"
    };
    sqlx::query("UPDATE price_source_posts SET date_status = ?, normalized_date = COALESCE(?, normalized_date) WHERE id = ?")
        .bind(status)
        .bind(&req.observation_date)
        .bind(&batch.source_post_id)
        .execute(pool)
        .await
        .map_err(map_db)?;
    crate::audit::log(pool, None, Some(actor), "confirm_batch_date", "price_import_batch", batch_id, None, None).await?;
    Ok(())
}

/// Approves a batch: lines still lacking verification stay unreviewed —
/// approval only closes the review workflow, it never verifies data by itself.
pub async fn set_batch_status(
    pool: &SqlitePool,
    actor: &str,
    batch_id: &str,
    status: &str,
) -> Result<ImportBatch, CommonError> {
    const OK: &[&str] = &["imported_needs_review", "in_review", "approved", "rejected"];
    if !OK.contains(&status) {
        return Err(CommonError::Validation(format!("invalid batch status: {status}")));
    }
    get_batch(pool, batch_id).await?;
    sqlx::query("UPDATE price_import_batches SET batch_status = ?, reviewed_at = ? WHERE id = ?")
        .bind(status)
        .bind(now_rfc3339())
        .bind(batch_id)
        .execute(pool)
        .await
        .map_err(map_db)?;
    crate::audit::log(pool, None, Some(actor), "set_batch_status", "price_import_batch", batch_id, None, None).await?;
    get_batch(pool, batch_id).await
}

/// Seeds the "أولاد زين" (Olad Zain) merchant + the user-supplied post as an
/// unreviewed import batch. Idempotent: checks by merchant slug.
pub async fn seed_olad_zain(pool: &SqlitePool, raw_post_text: &str) -> Result<(), CommonError> {
    const SLUG: &str = "awlad-zain";
    if merchants::find_by_slug(pool, SLUG).await?.is_some() {
        return Ok(());
    }
    let display_name = "محلات أولاد زين للخضروات والفواكة وجميع انواع الطيور الحية والمذبوحة وجميع انواع اللحوم المجمدة والمعجنات والتوابل والبقالة والماركت";
    let m = merchants::create_merchant(
        pool,
        &merchants::CreateMerchant {
            name: display_name.into(),
            slug: Some(SLUG.into()),
            mtype: Some("grocery".into()),
            phone: None,
            facebook_url: None, // not supplied; left null until added
            website_url: None,
            notes: Some("First seeded merchant from a user-supplied Facebook post text. City/area NOT supplied — must be confirmed by the user. Promotional claims in the post are merchant claims, not verified facts.".into()),
        },
    )
    .await?;
    let (_batch, _obs) = create_batch_from_text(
        pool,
        "system-seed",
        raw_post_text,
        "user_provided_facebook_post_text",
        None, // source URL unknown
        Some(&m.id),
    )
    .await?;
    tracing::info!("seeded Olad Zain merchant + unreviewed import batch");
    Ok(())
}
