//! Belhana API server: Axum router, migrations, seeding, CORS, tracing.

mod auth;
mod error;
mod routes;
mod seed;
mod seed_data;
mod state;

use std::sync::Arc;

use state::{AppState, Config};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,sqlx=warn".into()),
        )
        .init();

    let config = Config::from_env();
    let host = config.host.clone();
    let port = config.port;

    let pool = belhana_db::open_pool(&config.database_url)
        .await
        .expect("failed to create database pool");
    belhana_db::migrate(&pool).await.expect("failed to run migrations");
    seed::run(&pool).await.expect("failed to seed database");

    let state = Arc::new(AppState { pool, config });

    let app = routes_router(state.clone());

    let addr = format!("{host}:{port}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("failed to bind address");
    tracing::info!("belhana-api listening on http://{addr}");
    axum::serve(listener, app).await.expect("server error");
}

fn routes_router(state: Arc<AppState>) -> axum::Router {
    use axum::http::Method;
    use tower_http::cors::{Any, CorsLayer};
    use tower_http::trace::TraceLayer;

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(Any);

    axum::Router::new()
        .merge(routes::auth_routes::router())
        .merge(routes::households_routes::router())
        .merge(routes::family_routes::router())
        .merge(routes::rules_routes::router())
        .merge(routes::budget_routes::router())
        .merge(routes::catalog_routes::router())
        .merge(routes::pricing_routes::router())
        .merge(routes::merchants_routes::router())
        .merge(routes::imports_routes::router())
        .merge(routes::recipes_routes::router())
        .merge(routes::inventory_routes::router())
        .merge(routes::planning_routes::router())
        .merge(routes::shopping_routes::router())
        .merge(routes::events_routes::router())
        .merge(routes::ai_routes::router())
        .merge(routes::dashboard_routes::router())
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
