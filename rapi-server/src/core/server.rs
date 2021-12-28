use poem::{listener::TcpListener, Server, Endpoint};
use std::net::SocketAddr;

pub struct Application;

impl Application {
  pub async fn run<E: 'static + Endpoint>(routes: E) {
    tracing_subscriber::fmt::init();

    let addr: SocketAddr = format!("0.0.0.0:3000")
      .parse()
      .expect("Unable to resolve domain");
    Server::new(TcpListener::bind(&addr))
      .run(routes)
      .await.unwrap()
  }
}
