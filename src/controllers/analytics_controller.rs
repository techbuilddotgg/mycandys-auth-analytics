use crate::{repositories::analytics_repository::AnalyticsRepo};
use actix_web::{
    get, post,
    web::{Data, Json},
    HttpResponse,
};
use crate::models::analytics_model::Analytics;


#[get("/analytics")]
pub async fn get_all_analytics(db: Data<AnalyticsRepo>) -> HttpResponse {
    let analytics = db.get_all_analytics().await;
    
    match analytics {
        Ok(analytics) => HttpResponse::Ok().json(analytics),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/analytics")]
pub async fn create_analytics(db: Data<AnalyticsRepo>, new_analytics: Json<Analytics>) -> HttpResponse {
    let analytics = db.create_analytics(new_analytics.into_inner()).await;
    
    match analytics {
        Ok(analytics) => HttpResponse::Ok().json(analytics),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
