use crate::models::{Claims, User};
use crate::config::Config;
use anyhow::{Result, anyhow};
use bcrypt::{hash, verify};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

#[derive(Clone)]
pub struct AuthService {
    config: Config,
}

impl AuthService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn hash_password(&self, password: &str) -> Result<String> {
        let cost = self.config.bcrypt_cost;
        hash(password, cost).map_err(|e| anyhow!("Failed to hash password: {}", e))
    }

    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        verify(password, hash).map_err(|e| anyhow!("Failed to verify password: {}", e))
    }

    pub fn generate_token(&self, user: &User) -> Result<String> {
        let now = Utc::now();
        let exp = now + Duration::seconds(self.config.jwt_expiration);

        let claims = Claims {
            sub: user.id.to_string(),
            username: user.username.clone(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.jwt_secret.as_ref()),
        )
        .map_err(|e| anyhow!("Failed to generate token: {}", e))?;

        Ok(token)
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.config.jwt_secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|e| anyhow!("Failed to verify token: {}", e))?;

        Ok(token_data.claims)
    }

    pub fn extract_user_id_from_token(&self, token: &str) -> Result<String> {
        let claims = self.verify_token(token)?;
        Ok(claims.sub)
    }
}