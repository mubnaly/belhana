//! Database pool setup and migrations for Belhana.
//!
//! Development uses SQLite (embedded) with a Postgres-compatible schema design
//! (TEXT UUIDs, JSON-as-TEXT). The migration files live in
//! `database/migrations/sqlite/` and are embedded at compile time.

use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteRow};
use sqlx::{Row, SqlitePool};
use std::{path::PathBuf, str::FromStr, time::Duration};

/// Embedded migrations in application order.
pub const MIGRATIONS: &[(&str, &str)] = &[
    (
        "0001_auth_households",
        include_str!("../../../database/migrations/sqlite/0001_auth_households.sql"),
    ),
    (
        "0002_family_rules",
        include_str!("../../../database/migrations/sqlite/0002_family_rules.sql"),
    ),
    (
        "0003_catalog_units",
        include_str!("../../../database/migrations/sqlite/0003_catalog_units.sql"),
    ),
    (
        "0004_merchants_prices",
        include_str!("../../../database/migrations/sqlite/0004_merchants_prices.sql"),
    ),
    (
        "0005_recipes",
        include_str!("../../../database/migrations/sqlite/0005_recipes.sql"),
    ),
    (
        "0006_inventory",
        include_str!("../../../database/migrations/sqlite/0006_inventory.sql"),
    ),
    (
        "0007_planning",
        include_str!("../../../database/migrations/sqlite/0007_planning.sql"),
    ),
    (
        "0008_shopping",
        include_str!("../../../database/migrations/sqlite/0008_shopping.sql"),
    ),
    (
        "0009_occasions",
        include_str!("../../../database/migrations/sqlite/0009_occasions.sql"),
    ),
    (
        "0010_ai_ops",
        include_str!("../../../database/migrations/sqlite/0010_ai_ops.sql"),
    ),
    (
        "0011_guidance_and_purchasing",
        include_str!("../../../database/migrations/sqlite/0011_guidance_and_purchasing.sql"),
    ),
    (
        "0012_product_nutrition",
        include_str!("../../../database/migrations/sqlite/0012_product_nutrition.sql"),
    ),
    (
        "0013_product_macros",
        include_str!("../../../database/migrations/sqlite/0013_product_macros.sql"),
    ),
];

pub async fn open_pool(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    // Accept "sqlite://path.db" or a bare path.
    let mut path = database_url
        .strip_prefix("sqlite://")
        .unwrap_or(database_url)
        .to_string();
    if path.is_empty() {
        path = "belhana.db".to_string();
    }
    if let Some(parent) = PathBuf::from(&path).parent() {
        if !parent.as_os_str().is_empty() {
            let _ = std::fs::create_dir_all(parent);
        }
    }
    let opts = SqliteConnectOptions::from_str(&format!("sqlite://{path}"))?
        .create_if_missing(true)
        .foreign_keys(true)
        .journal_mode(SqliteJournalMode::Wal)
        .busy_timeout(Duration::from_secs(10));
    SqlitePoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(10))
        .connect_with(opts)
        .await
}

/// Applies any not-yet-applied migrations, tracked in `_belhana_migrations`.
pub async fn migrate(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS _belhana_migrations (name TEXT PRIMARY KEY, applied_at TEXT NOT NULL)",
    )
    .execute(pool)
    .await?;
    for (name, sql) in MIGRATIONS {
        let applied: Option<String> = sqlx::query("SELECT name FROM _belhana_migrations WHERE name = ?")
            .bind(name)
            .fetch_optional(pool)
            .await?
            .map(|r: SqliteRow| r.get("name"));
        if applied.is_some() {
            continue;
        }
        tracing::info!(migration = name, "applying migration");
        sqlx::query(sql).execute(pool).await?;
        sqlx::query("INSERT INTO _belhana_migrations (name, applied_at) VALUES (?, datetime('now'))")
            .bind(name)
            .execute(pool)
            .await?;
    }
    Ok(())
}

