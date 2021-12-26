mod server;
mod db;
pub mod result;
mod error;
pub use server::Application;
pub use db::connection;
pub use result::Res;
pub use error::AppError;