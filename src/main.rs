mod domain;
mod infrastructure;

use std::env;

use infrastructure::database::connection::create_pool;
use infrastructure::repository::user_repository::SqlxUserRepository;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = create_pool(&database_url).await?;

    let user_repository = SqlxUserRepository::new(pool);

    println!("Server initialized");

    Ok(())
}
