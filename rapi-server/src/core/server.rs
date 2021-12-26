use axum::Router;
use std::net::SocketAddr;

pub struct Application;

impl Application {
  pub async fn run(routes:Router) {
    tracing_subscriber::fmt::init();

    let addr: SocketAddr = format!("0.0.0.0:3000")
      .parse()
      .expect("Unable to resolve domain");
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
      .serve(routes.into_make_service())
      .await
      .unwrap();
  }
}
