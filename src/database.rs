use sqlx::{SqlitePool, Pool, Sqlite};
use anyhow::Result;

pub type DbPool = Pool<Sqlite>;

pub async fn create_pool(database_url: &str) -> Result<DbPool> {
    let pool = SqlitePool::connect(database_url).await?;
    Ok(pool)
}

pub async fn run_migrations(pool: &DbPool) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            email TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )"#,
    )
    .execute(pool)
    .await?;

    // 创建索引以提高查询性能
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_users_username ON users(username)")
        .execute(pool)
        .await?;
    
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_users_email ON users(email)")
        .execute(pool)
        .await?;

    Ok(())
}