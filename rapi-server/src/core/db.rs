use sqlx;
use sqlx::mysql::MySqlPool;

pub async fn connection(url: &str) -> MySqlPool {
  MySqlPool::connect(url).await.expect("数据库连接失败")
}
