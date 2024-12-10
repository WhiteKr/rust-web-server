use axum::extract::Query;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RectangleParams {
    height: f64,
    width: f64,
}

#[derive(Serialize)]
pub struct RectangleResponse {
    size: f64,
}

pub async fn calculate_rectangle(
    Query(params): Query<RectangleParams>,
) -> axum::Json<RectangleResponse> {
    let size = params.height * params.width;
    axum::Json(RectangleResponse { size })
}
