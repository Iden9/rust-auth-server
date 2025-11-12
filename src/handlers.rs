use crate::models::{LoginRequest, RegisterRequest};
use crate::services::UserService;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Result};
use serde::Serialize;

// 统一 API 返回实体
#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        ApiResponse {
            code: 200,
            message: "Success".to_string(),
            data: Some(data),
        }
    }

    pub fn error(message: String) -> ApiResponse<()> {
        ApiResponse {
            code: 500,
            message,
            data: None,
        }
    }
}

// 用户注册处理器
pub async fn register(
    user_service: web::Data<UserService>,
    req: web::Json<RegisterRequest>,
) -> Result<HttpResponse> {
    match user_service.register(req.into_inner()).await {
        Ok(auth_response) => Ok(HttpResponse::Ok().json(ApiResponse::success(auth_response))),
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()))),
    }
}

// 用户登录处理器
pub async fn login(
    user_service: web::Data<UserService>,
    req: web::Json<LoginRequest>,
) -> Result<HttpResponse> {
    match user_service.login(req.into_inner()).await {
        Ok(auth_response) => Ok(HttpResponse::Ok().json(ApiResponse::success(auth_response))),
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()))),
    }
}

// 获取用户信息处理器（需要认证）
pub async fn get_profile(
    user_service: web::Data<UserService>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    // 从请求扩展中获取用户ID
    let user_id = req.extensions().get::<String>().cloned()
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("User ID not found"))?;
    
    match user_service.get_user_by_id(&user_id).await {
        Ok(user) => Ok(HttpResponse::Ok().json(ApiResponse::success(user))),
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<String>::error(e.to_string()))),
    }
}

// 健康检查
pub async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(ApiResponse::success("Server is running")))
}