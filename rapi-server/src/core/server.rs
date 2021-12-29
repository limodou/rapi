use poem::{listener::TcpListener, Endpoint, Server};
use std::net::SocketAddr;

pub struct Application;

impl Application {
  pub async fn run<E: 'static + Endpoint>(routes: E) {
    tracing_subscriber::fmt::init();

    let host = match std::env::var_os("HOST") {
      Some(h) => h.to_str().unwrap().to_string(),
      None => "0.0.0.0".into(),
    };

    let port = match std::env::var_os("PORT") {
      Some(p) => p.to_str().unwrap().to_string(),
      None => "3000".into(),
    };

    let addr: SocketAddr = format!("{}:{}", host, port)
      .parse()
      .expect("Unable to resolve domain");
    Server::new(TcpListener::bind(&addr))
      .run(routes)
      .await
      .unwrap()
  }
}
