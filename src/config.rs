use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiration: i64,
    pub server_host: String,
    pub server_port: u16,
    pub bcrypt_cost: u32,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv::dotenv().ok();
        
        Ok(Config {
            database_url: env::var("DATABASE_URL")?,
            jwt_secret: env::var("JWT_SECRET")?,
            jwt_expiration: env::var("JWT_EXPIRATION")
                .unwrap_or_else(|_| "86400".to_string())
                .parse()
                .unwrap_or(86400),
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            bcrypt_cost: env::var("BCRYPT_COST")
                .unwrap_or_else(|_| "12".to_string())
                .parse()
                .unwrap_or(12),
        })
    }
}