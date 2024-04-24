use axum::routing::{get, post, put};

use crate::handler::user;
use crate::server::state::AppState;

pub fn add_routers(router: axum::Router<AppState>) -> axum::Router<AppState> {
    router
        .route("/api/v1/report/register", post(user::register))
        .route("/api/v1/report/active", put(user::active))
}
