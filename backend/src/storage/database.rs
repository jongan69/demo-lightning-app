use sqlx::{PgPool, Pool, Postgres};
use anyhow::Result;
use tracing::info;

pub async fn init_db() -> Result<PgPool> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:password@localhost:5432/taproot_assets".to_string());
    
    info!("Connecting to database: {}", database_url);
    
    let pool = PgPool::connect(&database_url).await?;
    
    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    Ok(pool)
}

pub async fn get_asset_balance(pool: &PgPool, asset_id: &str) -> Result<u64> {
    let row = sqlx::query!(
        "SELECT balance FROM asset_balances WHERE asset_id = $1",
        asset_id
    )
    .fetch_optional(pool)
    .await?;
    
    Ok(row.map(|r| r.balance as u64).unwrap_or(0))
}

pub async fn update_asset_balance(pool: &PgPool, asset_id: &str, balance: u64) -> Result<()> {
    sqlx::query!(
        "INSERT INTO asset_balances (asset_id, balance) VALUES ($1, $2)
         ON CONFLICT (asset_id) DO UPDATE SET balance = $2, updated_at = NOW()",
        asset_id,
        balance as i64
    )
    .execute(pool)
    .await?;
    
    Ok(())
}