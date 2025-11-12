use crate::auth::AuthService;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    rc::Rc,
};

#[derive(Clone)]
pub struct AuthMiddleware {
    auth_service: Rc<AuthService>,
}

impl AuthMiddleware {
    pub fn new(auth_service: AuthService) -> Self {
        Self {
            auth_service: Rc::new(auth_service),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service: Rc::new(service),
            auth_service: self.auth_service.clone(),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
    auth_service: Rc<AuthService>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let auth_service = self.auth_service.clone();
        let service = self.service.clone();
        
        Box::pin(async move {
            // 检查是否需要认证的路径
            let path = req.path();
            let needs_auth = path.starts_with("/api/profile") || path.starts_with("/api/protected");
            
            if !needs_auth {
                let res = service.call(req).await?;
                return Ok(res);
            }

            // 提取 Authorization header
            let auth_header = req
                .headers()
                .get("Authorization")
                .and_then(|h| h.to_str().ok())
                .and_then(|h| {
                    if h.starts_with("Bearer ") {
                        Some(&h[7..])
                    } else {
                        None
                    }
                });

            let token = match auth_header {
                Some(token) => token.to_string(),
                None => {
                    let response = HttpResponse::Unauthorized()
                        .json(serde_json::json!({
                            "code": 401,
                            "message": "Missing or invalid authorization header",
                            "data": null
                        }));
                    return Ok(req.error_response(response));
                }
            };

            // 验证 token
            match auth_service.verify_token(&token) {
                Ok(claims) => {
                    // 将用户 ID 添加到请求扩展中
                    req.extensions_mut().insert(claims.sub.clone());
                    
                    let res = service.call(req).await?;
                    Ok(res)
                }
                Err(_) => {
                    let response = HttpResponse::Unauthorized()
                        .json(serde_json::json!({
                            "code": 401,
                            "message": "Invalid or expired token",
                            "data": null
                        }));
                    Ok(req.error_response(response))
                }
            }
        })
    }
}