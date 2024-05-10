use axum::{routing::post, Router};
use routes::password::generate_password_link;

mod routes;
mod services;
mod utils;

#[tokio::main]
async fn main() {
    // initialize tracing

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        // `POST /users` goes to `create_user`
        .route("/generate-password-link", post(generate_password_link));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
