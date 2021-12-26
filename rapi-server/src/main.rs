use axum::{routing::post, AddExtensionLayer, Router};
mod auth;
mod core;

use crate::auth::{login, register};
use crate::core::{connection, Application};

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "rapi-server=debug,tower_http=debug")
    }

    let pool = connection("mysql://root:rootroot@localhost:3306/rapi").await;

    let app = Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(AddExtensionLayer::new(pool));

    Application::run(app).await;
}
