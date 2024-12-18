use sqlx::{Executor, PgPool};

#[tokio::main]
async fn main() {
   let pool = PgPool::connect("postgres://developer:developer@localhost/sample").await.unwrap();

   let result = pool.execute("SELECT 1").await.unwrap();

   println!("Result: {:?}", result.rows_affected());
}
