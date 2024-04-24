use fake::Dummy;
use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Dummy, Validate, utoipa::ToSchema)]
pub struct ReportRegisterRequest {
    #[dummy]
    #[garde(range(min = 1))]
    pub category_id: i32,

    #[dummy(faker = "Text(10..200)")]
    #[garde(length(max = 200))]
    pub description: Option<String>,

    #[dummy]
    pub duration: u32,

    #[dummy]
    pub location_lat: f64,

    #[dummy]
    pub location_long: f64,
}