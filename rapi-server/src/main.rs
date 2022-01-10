use dotenv::dotenv;
use poem::{
    endpoint::StaticFiles,
    get, handler,
    middleware::{AddData, Tracing},
    post,
    web::Data,
    EndpointExt, Route,
};

mod core;
mod utils;
mod auth; // 认证模块
use auth::jwt_middle::{JwtTokenMiddleware, UserId};
use auth::jwt_token::{JwtUser, JwtUserNotCheck};
mod group; // 分组模块

#[handler]
fn hello(JwtUser(user): JwtUser) -> String {
    println!("{:?}", user);
    format!("hello: Poem {:?}", user)
}

#[handler]
fn hello1(JwtUserNotCheck(user): JwtUserNotCheck) -> String {
    println!("{:?}", user);
    format!("hello: Poem {:?}", user)
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
        .at("/hello", get(hello))
        .at("/hello1", get(hello1))
        .at("/api/login", post(auth::controller::login))
        .at("/api/register", post(auth::controller::register))
        .at("/api/getUserInfo", post(auth::controller::get_user_info))
        .at("/api/createGroup", post(group::controller::create_group))
        .nest("/", StaticFiles::new(static_dir).index_file("index.html"))
        .with(Tracing)
        .with(AddData::new(pool))
        .with(JwtTokenMiddleware)
        .catch_all_error(core::error::custom_error);

    core::Application::run(app).await;
}
