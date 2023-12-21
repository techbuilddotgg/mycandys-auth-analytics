use std::env;
extern crate dotenv;

use dotenv::dotenv;

use futures::stream::TryStreamExt;
use mongodb::{
    bson::{extjson::de::Error},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};
use chrono::{Utc, SecondsFormat};


use crate::models::analytics_model::{Analytics, CreateAnalyticsDto};

pub struct AnalyticsRepo {
    col: Collection<Analytics>,
}

impl AnalyticsRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGO_URI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
    
        let client = Client::with_uri_str(uri)
            .await
            .expect("error connecting to database");
        let db = client.database("auth-analytics");
        let col: Collection<Analytics> = db.collection("analytics");

        println!("Connected to database");

        AnalyticsRepo { col }
    }
    pub async fn get_all_analytics(&self) -> Result<Vec<Analytics>, Error> {
        let mut cursors = self
            .col
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of analytics");
        let mut analytics: Vec<Analytics> = Vec::new();
        while let Some(analytic) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            analytics.push(analytic)
        }
        Ok(analytics)
    }
    pub async fn create_analytics(&self, new_analytics: CreateAnalyticsDto) -> Result<InsertOneResult, Error> {
        let current_time = Utc::now();
        let iso_time = current_time.to_rfc3339_opts(SecondsFormat::Secs, true); // Pridobite ISO niz časa

        let new_doc = Analytics {
            id: None,
            endpoint: new_analytics.endpoint,
            timestamp: iso_time,
        };
        let analytics = self
            .col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating analytics document");

        Ok(analytics)
    }
}