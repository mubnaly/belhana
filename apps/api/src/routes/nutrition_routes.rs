//! Nutrition guidance and athlete-planning endpoints.
//!
//! Educational estimates only: no personal profile is stored and no medical
//! advice is given. See `docs/19-NUTRITION-AND-SPORT-EVIDENCE.md`.

use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde_json::json;

use crate::auth::AuthUser;
use crate::error::ApiResult;
use crate::state::SharedState;
use belhana_core::guidance;

async fn guide(_state: State<SharedState>, _user: AuthUser) -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(json!(guidance::guide())))
}

async fn athlete_plan(
    _state: State<SharedState>,
    _user: AuthUser,
    Json(input): Json<guidance::AthleteInput>,
) -> ApiResult<Json<serde_json::Value>> {
    let plan = guidance::athlete_plan(&input)?;
    Ok(Json(json!(plan)))
}

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/api/nutrition-guide", get(guide))
        .route("/api/nutrition/athlete-plan", post(athlete_plan))
}
