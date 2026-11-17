//! Belhana maintenance worker: periodic background jobs.
//!
//! - Stale price sweep: observations older than N days (default 30) become
//!   `stale` per the price-integrity policy.
//! - Expiry notifications: warns households about stock expiring within 3 days
//!   and never re-notifies for the same lot within the window.
//!
//! Run standalone (`maintenance_worker`) or via `--once` for a single pass
//! (useful for cron/scheduled tasks).

use std::time::Duration;

use belhana_common::CommonError;
use sqlx::{Row, SqlitePool};

const SWEEP_INTERVAL_MINUTES: u64 = 60;
const STALE_DAYS_DEFAULT: i64 = 30;
const EXPIRY_WARN_DAYS: i64 = 3;

async fn sweep_stale_prices(pool: &SqlitePool) -> Result<u64, CommonError> {
    let days: i64 = std::env::var("BELHANA_STALE_DAYS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(STALE_DAYS_DEFAULT);
    let n = belhana_core::pricing::sweep_stale(pool, days).await?;
    if n > 0 {
        tracing::info!("stale price sweep: marked {n} observations as stale (> {days} days)");
    }
    Ok(n)
}

/// One notification per household+lot pair inside the warn window.
async fn expiry_notifications(pool: &SqlitePool) -> Result<usize, CommonError> {
    let rows = sqlx::query(
        "SELECT s.id, s.household_id, s.product_id, s.expiry_date, s.quantity, s.unit, p.name_ar
         FROM stock_lots s LEFT JOIN products p ON p.id = s.product_id
         WHERE s.status = 'active' AND s.expiry_date IS NOT NULL
           AND s.expiry_date != '' AND s.quantity > 0",
    )
    .fetch_all(pool)
    .await
    .map_err(belhana_core::map_db)?;

    let today = belhana_core::today();
    let mut created = 0usize;
    for r in rows {
        let expiry: String = r.get("expiry_date");
        let Ok(exp_date) = chrono::NaiveDate::parse_from_str(&expiry, "%Y-%m-%d") else {
            continue;
        };
        let days_left = (exp_date - today).num_days();
        if days_left > EXPIRY_WARN_DAYS {
            continue;
        }
        let household_id: String = r.get("household_id");
        let lot_id: String = r.get("id");
        // avoid duplicates: same kind + lot id referenced in title
        let title = format!("expiry_reminder:{lot_id}");
        let exists: Option<String> = sqlx::query_scalar(
            "SELECT id FROM notifications WHERE household_id = ? AND kind = 'expiry_reminder' AND title = ?",
        )
        .bind(&household_id)
        .bind(&title)
        .fetch_optional(pool)
        .await
        .map_err(belhana_core::map_db)?;
        if exists.is_some() {
            continue;
        }
        let product: Option<String> = r.get("product_name_ar");
        let product_name = product.unwrap_or_else(|| "منتج".into());
        let qty: f64 = r.get("quantity");
        let unit: String = r.get("unit");
        let body = if days_left < 0 {
            format!("{product_name} ({qty} {unit}) انتهت صلاحيته بتاريخ {expiry} — تخلّص منه أو سجّل الهدر.")
        } else if days_left == 0 {
            format!("{product_name} ({qty} {unit}) صلاحيته تنتهي اليوم — استخدمه فورًا.")
        } else {
            format!("{product_name} ({qty} {unit}) تنتهي صلاحيته خلال {days_left} أيام ({expiry}).")
        };
        sqlx::query(
            "INSERT INTO notifications (id, household_id, kind, title, body, read, created_at)
             VALUES (?, ?, 'expiry_reminder', ?, ?, 0, ?)",
        )
        .bind(belhana_core::new_id())
        .bind(&household_id)
        .bind(&title)
        .bind(&body)
        .bind(belhana_core::now_rfc3339())
        .execute(pool)
        .await
        .map_err(belhana_core::map_db)?;
        created += 1;
    }
    if created > 0 {
        tracing::info!("expiry notifications: created {created}");
    }
    Ok(created)
}

/// Removes notifications older than 30 days to keep the list tidy.
async fn prune_notifications(pool: &SqlitePool) -> Result<u64, CommonError> {
    let cutoff = (chrono::Utc::now() - chrono::Duration::days(30)).to_rfc3339();
    let res = sqlx::query("DELETE FROM notifications WHERE created_at < ?")
        .bind(&cutoff)
        .execute(pool)
        .await
        .map_err(belhana_core::map_db)?;
    Ok(res.rows_affected())
}

async fn run_once(pool: &SqlitePool) -> Result<(), CommonError> {
    sweep_stale_prices(pool).await?;
    expiry_notifications(pool).await?;
    prune_notifications(pool).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,sqlx=warn".into()),
        )
        .init();

    let database_url =
        std::env::var("BELHANA_DATABASE_URL").unwrap_or_else(|_| "belhana.db".into());
    let pool = belhana_db::open_pool(&database_url)
        .await
        .expect("failed to open database");
    belhana_db::migrate(&pool).await.expect("failed to migrate");

    let once = std::env::args().any(|a| a == "--once");
    if once {
        if let Err(e) = run_once(&pool).await {
            tracing::error!("maintenance pass failed: {e}");
            std::process::exit(1);
        }
        tracing::info!("maintenance pass complete");
        return;
    }

    tracing::info!("maintenance worker running every {SWEEP_INTERVAL_MINUTES} minutes");
    let mut ticker = tokio::time::interval(Duration::from_secs(SWEEP_INTERVAL_MINUTES * 60));
    ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
    loop {
        ticker.tick().await;
        if let Err(e) = run_once(&pool).await {
            tracing::error!("maintenance pass failed: {e}");
        }
    }
}
