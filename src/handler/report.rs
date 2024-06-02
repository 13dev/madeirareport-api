use std::str::FromStr;
use axum::extract::{State};
use axum::Json;
use garde::Validate;
use tracing::{info, warn};
use uuid::Uuid;

use crate::error::AppResult;
use crate::server::state::AppState;
use crate::{service};
use crate::dto::report::{ReportRegisterRequest, ReportRegisterResponse};

/// Register new request.
#[utoipa::path(
    post,
    request_body = ReportRegisterRequest,
    path = "/api/v1/report/register",
    responses(
        (status = 200, description = "Success register report", body = [ReportRegisterResponse]),
        (status = 400, description = "Invalid data input", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError])
    )
)]
pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<ReportRegisterRequest>,
) -> AppResult<Json<ReportRegisterResponse>> {

    info!("Register new report with request: {req:?}");

    req.validate(&())?;
    match service::report::register(state, req).await {
        Ok(user_id) => {
            info!("Successfully register report");
            Ok(Json(ReportRegisterResponse { id: Uuid::from_str("sa")? }))
        }
        Err(e) => {
            warn!("Unsuccessfully register user: {e:?}");
            Err(e)
        }
    }
}