mod config;
mod handlers;
mod models;

use actix_web::{web, App, HttpServer, middleware};
use models::AppState;
use std::sync::Mutex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(AppState { users: Mutex::new(vec![]) });

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(data.clone())
            .service(
                web::resource("/users")
                    .route(web::post().to(handlers::create_user))
                    .route(web::get().to(|| async { "Use /users/{id} to get a user" })),
            )
            .service(
                web::resource("/users/{id}")
                    .route(web::get().to(handlers::get_user))
                    .route(web::put().to(handlers::update_user))
                    .route(web::delete().to(handlers::delete_user)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}