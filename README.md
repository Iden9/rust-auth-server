# Rust 用户认证服务器

这是一个基于 Rust 和 Actix-web 构建的用户认证服务器，提供用户注册、登录功能，使用 PostgreSQL 数据库和 JWT 认证。

## 功能特性

- ✅ 用户注册和登录
- ✅ 密码加密（bcrypt）
- ✅ JWT 令牌认证
- ✅ PostgreSQL 数据库集成
- ✅ 统一的 API 响应格式
- ✅ 环境变量配置

## 技术栈

- **Web 框架**: Actix-web 4.11
- **数据库**: PostgreSQL (通过 SQLx)
- **认证**: JWT + bcrypt
- **序列化**: Serde
- **异步运行时**: Tokio

## 快速开始

### 1. 环境准备

确保你已经安装了：
- Rust (1.86+)
- PostgreSQL

### 2. 数据库设置

创建 PostgreSQL 数据库：
```sql
CREATE DATABASE rust_auth_db;
CREATE USER username WITH PASSWORD 'password';
GRANT ALL PRIVILEGES ON DATABASE rust_auth_db TO username;
```

### 3. 配置环境变量

复制并修改 `.env` 文件：
```bash
# 数据库配置
DATABASE_URL=postgresql://username:password@localhost/rust_auth_db

# JWT 配置
JWT_SECRET=your-super-secret-jwt-key-change-this-in-production
JWT_EXPIRATION=86400

# 服务器配置
SERVER_HOST=127.0.0.1
SERVER_PORT=8080

# 密码加密配置
BCRYPT_COST=12
```

### 4. 运行服务器

```bash
# 编译并运行
cargo run

# 或者使用 cargo watch 进行开发
cargo install cargo-watch
cargo watch -x run
```

服务器将在 `http://127.0.0.1:8080` 启动。

## API 接口

### 健康检查
```http
GET /api/health
```

响应：
```json
{
  "code": 200,
  "message": "Success",
  "data": "Server is running"
}
```

### 用户注册
```http
POST /api/register
Content-Type: application/json

{
  "username": "testuser",
  "email": "test@example.com",
  "password": "password123"
}
```

响应：
```json
{
  "code": 200,
  "message": "Success",
  "data": {
    "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
    "user": {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "username": "testuser",
      "email": "test@example.com",
      "created_at": "2024-01-01T00:00:00Z"
    }
  }
}
```

### 用户登录
```http
POST /api/login
Content-Type: application/json

{
  "username": "testuser",
  "password": "password123"
}
```

响应格式与注册相同。

## 数据库结构

### users 表
```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

## 项目结构

```
src/
├── auth.rs          # JWT 认证服务
├── config.rs        # 配置管理
├── database.rs      # 数据库连接和迁移
├── handlers.rs      # HTTP 请求处理器
├── lib.rs          # 模块声明
├── models.rs       # 数据模型
├── services.rs     # 业务逻辑服务
└── middleware.rs   # 中间件（待完善）
main.rs             # 应用入口点
.env                # 环境变量配置
Cargo.toml          # 项目依赖
```

## 开发说明

### 添加新的 API 端点

1. 在 `models.rs` 中定义请求/响应结构
2. 在 `services.rs` 中实现业务逻辑
3. 在 `handlers.rs` 中添加 HTTP 处理器
4. 在 `main.rs` 中注册路由

### 数据库迁移

数据库表会在应用启动时自动创建。如需修改表结构，请更新 `database.rs` 中的 `run_migrations` 函数。

## 安全注意事项

- 🔒 请在生产环境中更改 `JWT_SECRET`
- 🔒 使用强密码策略
- 🔒 启用 HTTPS
- 🔒 定期更新依赖项
- 🔒 配置适当的 CORS 策略

## 许可证

MIT License