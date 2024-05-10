use axum::Json;
use axum_extra::extract::WithRejection;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_email::Email;

use crate::utils::ApiError;

#[derive(Deserialize)]
pub struct PasswordLinkRequest {
    pub email: Email,
}
#[derive(Serialize)]
pub struct PasswordLinkResponse {
    pub token: String,
}

pub async fn generate_password_link(
    // Json(payload): Json<PasswordLinkRequest>,
    WithRejection(Json(payload), _): WithRejection<Json<PasswordLinkRequest>, ApiError>,
) -> (StatusCode, Json<PasswordLinkResponse>) {
    let token = PasswordLinkResponse {
        token: payload.email.to_string(),
    };

    (StatusCode::CREATED, Json(token))
}
