use axum::extract::{Query, State};
use axum::Json;
use garde::Validate;
use tracing::{info, warn};

use crate::error::AppResult;
use crate::server::state::AppState;
use crate::util::claim::UserClaims;
use crate::{dto::*, service};

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
) -> AppResult<Json<ReportRegisterRequest>> {

    info!("Register new report with request: {req:?}");

    req.validate(&())?;
    match service::report::register(state, req).await {
        Ok(user_id) => {
            info!("Successfully register user: {user_id}");
            let resp = ReportRegisterRequest { id: user_id };
            Ok(Json(resp))
        }
        Err(e) => {
            warn!("Unsuccessfully register user: {e:?}");
            Err(e)
        }
    }
}