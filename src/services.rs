use crate::auth::AuthService;
use crate::database::DbPool;
use crate::models::{AuthResponse, LoginRequest, RegisterRequest, User, UserResponse};
use anyhow::{anyhow, Result};
use uuid::Uuid;

#[derive(Clone)]
pub struct UserService {
    pool: DbPool,
    auth_service: AuthService,
}

impl UserService {
    pub fn new(pool: DbPool, auth_service: AuthService) -> Self {
        Self { pool, auth_service }
    }

    pub async fn register(&self, req: RegisterRequest) -> Result<AuthResponse> {
        // 检查用户名是否已存在
        let existing_user = sqlx::query("SELECT id FROM users WHERE username = $1 OR email = $2")
            .bind(&req.username)
            .bind(&req.email)
            .fetch_optional(&self.pool)
            .await?;

        if existing_user.is_some() {
            return Err(anyhow!("Username or email already exists"));
        }

        // 验证输入
        if req.username.len() < 3 || req.username.len() > 50 {
            return Err(anyhow!("Username must be between 3 and 50 characters"));
        }

        if req.password.len() < 6 {
            return Err(anyhow!("Password must be at least 6 characters"));
        }

        if !req.email.contains('@') {
            return Err(anyhow!("Invalid email format"));
        }

        // 加密密码
        let password_hash = self.auth_service.hash_password(&req.password)?;

        // 生成 UUID
        let user_id = Uuid::new_v4().to_string();
        
        // 创建用户
        sqlx::query(
            r#"
            INSERT INTO users (id, username, email, password_hash)
            VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(&user_id)
        .bind(&req.username)
        .bind(&req.email)
        .bind(&password_hash)
        .execute(&self.pool)
        .await?;

        // 获取创建的用户
        let user = sqlx::query_as::<_, User>(
            "SELECT id, username, email, password_hash, created_at, updated_at FROM users WHERE id = $1"
        )
        .bind(&user_id)
        .fetch_one(&self.pool)
        .await?;

        // 生成 JWT token
        let token = self.auth_service.generate_token(&user)?;

        Ok(AuthResponse {
            token,
            user: user.into(),
        })
    }

    pub async fn login(&self, req: LoginRequest) -> Result<AuthResponse> {
        // 查找用户
        let user = sqlx::query_as::<_, User>(
            "SELECT id, username, email, password_hash, created_at, updated_at FROM users WHERE username = $1"
        )
        .bind(&req.username)
        .fetch_optional(&self.pool)
        .await?;

        let user = user.ok_or_else(|| anyhow!("Invalid username or password"))?;

        // 验证密码
        if !self.auth_service.verify_password(&req.password, &user.password_hash)? {
            return Err(anyhow!("Invalid username or password"));
        }

        // 生成 JWT token
        let token = self.auth_service.generate_token(&user)?;

        Ok(AuthResponse {
            token,
            user: user.into(),
        })
    }

    pub async fn get_user_by_id(&self, user_id: &str) -> Result<UserResponse> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, username, email, password_hash, created_at, updated_at FROM users WHERE id = $1"
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        let user = user.ok_or_else(|| anyhow!("User not found"))?;
        Ok(user.into())
    }
}