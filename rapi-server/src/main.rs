use dotenv::dotenv;
use poem::{get, handler, middleware::AddData, post, web::Data, EndpointExt, Route};

mod auth;
mod core;
mod utils;
use utils::jwt_middle::{JwtTokenMiddleware, User};

#[handler]
fn hello(Data(user): Data<&User>) -> String {
    println!("{:?}", user);
    format!("hello: Poem {:?}", user.0.clone())
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }

    let connection = match std::env::var_os("DB_CONNECTION") {
        Some(c) => c.to_str().unwrap().to_string(),
        None => panic!("DB_CONNECTION is not set in env"),
    };

    let pool = core::connection(&connection).await;

    let app = Route::new()
        .at("/", get(hello))
        .at("/login", post(auth::controller::login))
        .at("/register", post(auth::controller::register))
        .with(AddData::new(pool))
        .with(JwtTokenMiddleware)
        .catch_all_error(core::error::custom_error);

    core::Application::run(app).await;
}
