use fake::Dummy;
use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Dummy, Validate, utoipa::ToSchema)]
pub struct ReportRegisterRequest {
    #[dummy]
    #[garde(range(min = 1))]
    pub category_id: i32,

    #[garde(ascii, length(min = 10, max = 200))]
    pub description: Option<String>,

    #[dummy]
    #[garde(skip)]
    pub duration: u32,

    #[dummy]
    #[garde(skip)]
    pub location_lat: f64,

    #[dummy]
    #[garde(skip)]
    pub location_long: f64,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Dummy)]
pub struct ReportRegisterResponse {
    pub id: Uuid,
}