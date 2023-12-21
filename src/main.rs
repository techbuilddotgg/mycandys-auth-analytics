mod controllers;
mod models;
mod repositories;

use actix_web::{web::Data, App, HttpServer};
use controllers::analytics_controller::{get_all_analytics, create_analytics};
use repositories::analytics_repository::AnalyticsRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = AnalyticsRepo::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(get_all_analytics)
            .service(create_analytics)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}