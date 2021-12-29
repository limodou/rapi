use dotenv::dotenv;
use poem::{
    endpoint::StaticFiles, get, handler, middleware::AddData, post, web::Data, EndpointExt, Route,
};

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

    let static_dir = match std::env::var_os("STATIC_DIR") {
        Some(c) => c.to_str().unwrap().to_string(),
        None => "./static".into(),
    };

    let pool = core::connection(&connection).await;

    let app = Route::new()
        // .at("/", get(hello))
        .at("/api/login", post(auth::controller::login))
        .at("/api/register", post(auth::controller::register))
        .nest("/", StaticFiles::new(static_dir).index_file("index.html"))
        .with(AddData::new(pool))
        .with(JwtTokenMiddleware)
        .catch_all_error(core::error::custom_error);

    core::Application::run(app).await;
}
