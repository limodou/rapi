mod server;
mod db;
pub mod result;
pub mod error;
pub use server::Application;
pub use db::connection;
pub use result::Res;
