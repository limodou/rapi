use dotenv::dotenv;
use poem::{
    get, handler, middleware::AddData, post, web::Data, EndpointExt, Error, IntoResponse, Route,
};

mod auth;
mod core;
mod utils;
use utils::jwt_middle::{JwtTokenMiddleware, User, TokenParseError};

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
    let pool = core::connection("mysql://root:rootroot@localhost:3306/rapi").await;

    let app = Route::new()
        .at("/", get(hello))
        .at("/login", post(auth::controller::login))
        .at("/register", post(auth::controller::register))
        .with(AddData::new(pool))
        .with(JwtTokenMiddleware)
        .catch_all_error(custom_error);

    core::Application::run(app).await;
}

async fn custom_error(err: Error) -> impl IntoResponse {
    //  Json(ErrorResponse {
    //      message: err.to_string(),
    //  })
    println!("{}", err.is::<TokenParseError>());
    core::result::fail(1000, &err.to_string())
}
