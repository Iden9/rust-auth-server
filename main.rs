use actix_web::{middleware::Logger, web, App, HttpServer};
use rs::{
    auth::AuthService,
    config::Config,
    database::{create_pool, run_migrations},
    handlers::{health_check, login, register},
    services::UserService,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志
    env_logger::init();

    // 加载配置
    let config = Config::from_env().expect("Failed to load configuration");
    println!("Starting server at {}:{}", config.server_host, config.server_port);

    // 创建数据库连接池
    let pool = create_pool(&config.database_url)
        .await
        .expect("Failed to create database pool");

    // 运行数据库迁移
    run_migrations(&pool)
        .await
        .expect("Failed to run database migrations");

    // 创建服务
    let auth_service = AuthService::new(config.clone());
    let user_service = UserService::new(pool, auth_service.clone());

    let server_host = config.server_host.clone();
    let server_port = config.server_port;

    // 启动服务器
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(user_service.clone()))
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .route("/health", web::get().to(health_check))
                    .route("/register", web::post().to(register))
                    .route("/login", web::post().to(login))
            )
    })
    .bind((server_host, server_port))?
    .run()
    .await
}
