use sqlx::PgPool;
use anyhow::Result;
use tracing::info;

#[allow(dead_code)]
pub async fn create_pool() -> Result<PgPool> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:password@localhost:5432/taproot_assets".to_string());
    
    info!("Connecting to database: {}", database_url);
    
    let pool = PgPool::connect(&database_url).await?;
    
    // TODO: Run migrations in production
    // sqlx::migrate!("./migrations").run(&pool).await?;
    
    Ok(pool)
}

#[allow(dead_code)]
pub async fn get_asset_balance(pool: &PgPool, asset_id: &str) -> Result<u64> {
    let row = sqlx::query_as::<_, (Option<i64>,)>(
        "SELECT balance FROM asset_balances WHERE asset_id = $1"
    )
    .bind(asset_id)
    .fetch_optional(pool)
    .await?;
    
    Ok(row.map(|r| r.0.unwrap_or(0) as u64).unwrap_or(0))
}

#[allow(dead_code)]
pub async fn update_asset_balance(pool: &PgPool, asset_id: &str, balance: u64) -> Result<()> {
    sqlx::query(
        "INSERT INTO asset_balances (asset_id, balance) VALUES ($1, $2)
         ON CONFLICT (asset_id) DO UPDATE SET balance = $2, updated_at = NOW()"
    )
    .bind(asset_id)
    .bind(balance as i64)
    .execute(pool)
    .await?;
    
    Ok(())
}