use axum::{routing::post, Router};
use routes::password::generate_password_link;
mod models;
mod routes;
mod services;
mod utils;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/generate-password-link", post(generate_password_link));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
