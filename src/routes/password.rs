use axum::Json;
use axum_extra::extract::WithRejection;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_email::Email;

use crate::{models::auth0::Auth0User, services::auth0::find_user_by_email, utils::ApiError};

#[derive(Deserialize)]
pub struct PasswordLinkRequest {
    pub email: Email,
}
#[derive(Serialize)]
pub struct PasswordLinkResponse {
    pub token: String,
}

pub async fn generate_password_link(
    WithRejection(Json(payload), _): WithRejection<Json<PasswordLinkRequest>, ApiError>,
) -> (StatusCode, Json<Vec<Auth0User>>) {
    let email = payload.email.to_string();
    let users = find_user_by_email(&email).await.unwrap();

    (StatusCode::CREATED, Json(users))
}
