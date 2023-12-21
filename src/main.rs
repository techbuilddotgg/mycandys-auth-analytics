mod controllers;
mod models;
mod repositories;

use actix_web::{web::Data, App, HttpServer};
use controllers::analytics_controller::{get_all_analytics, create_analytics, get_latest_analytics, get_most_called_endpoint, get_number_of_call_for_each_endpoint};
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
            .service(get_latest_analytics)
            .service(get_most_called_endpoint)
            .service(get_number_of_call_for_each_endpoint)
    })
    .bind(("0.0.0.0", 10000))?
    .run()
    .await
}