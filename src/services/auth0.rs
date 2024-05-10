use crate::models::auth0::Auth0Users;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

use dotenv::dotenv;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TOKEN_URL: String = set_token_url();
    pub static ref API_AUDIENCE: String = set_api_audience();
    pub static ref USERS_BY_EMAIL: String = set_users_by_email();
}

fn set_token_url() -> String {
    dotenv().ok();
    format!("https://{}/oauth/token", env::var("AUTH0_DOMAIN").unwrap())
}
fn set_api_audience() -> String {
    dotenv().ok();
    format!("https://{}/api/v2/", env::var("AUTH0_DOMAIN").unwrap())
}
fn set_users_by_email() -> String {
    dotenv().ok();
    format!(
        "https://{}/api/v2/users-by-email",
        env::var("AUTH0_DOMAIN").unwrap()
    )
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Auth0Token {
    access_token: String,
}

async fn get_auth0_token_cache() -> Result<Auth0Token, reqwest::Error> {
    todo!("Implement the logic to cache the Auth0 token")
}

// Function to get an Auth0 API token
async fn get_auth0_token() -> Result<Auth0Token, reqwest::Error> {
    let client = Client::new();
    let response = client
        .post(TOKEN_URL.to_string())
        .form(&[
            ("client_id", env::var("AUTH0_CLIENT_ID").unwrap()),
            ("client_secret", env::var("AUTH0_CLIENT_SECRET").unwrap()),
            ("audience", API_AUDIENCE.to_string()),
            ("grant_type", "client_credentials".to_string()),
        ])
        .send()
        .await?
        .json::<Auth0Token>()
        .await?;

    Ok(response)
}

pub async fn find_user_by_email(email: &str) -> Result<Auth0Users, reqwest::Error> {
    let token: Auth0Token = get_auth0_token().await.unwrap();
    let client = Client::new();
    let response = client
        .get(USERS_BY_EMAIL.to_string())
        .query(&[("email", email)])
        .bearer_auth(&token.access_token)
        .send()
        .await?
        .json::<Auth0Users>()
        .await?;

    // Print the entire JSON response
    // println!("{:#?}", response);

    Ok(response)
}
