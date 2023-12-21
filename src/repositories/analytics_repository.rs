use std::env;
extern crate dotenv;

use dotenv::dotenv;

use futures::stream::TryStreamExt;
use mongodb::{
    bson::{extjson::de::Error, doc, Document},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    options::{FindOneOptions, FindOptions, UpdateOptions},
    Client, Collection,
};
use chrono::{Utc, SecondsFormat};
use std::io::ErrorKind;
use bson;


use crate::models::analytics_model::{Analytics, CreateAnalyticsDto, CountEndpointsReponseDto, MyDocument};

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
    pub async fn get_latest_analytics(&self) -> Result<Option<Analytics>, Error> {
        let options = FindOneOptions::builder()
            .sort(doc! { "timestamp": -1 })
            .build();
        let analytics = self
            .col
            .find_one(None, options)
            .await
            .ok()
            .expect("Error getting latest analytics");

        Ok(analytics)
    }

    pub async fn get_most_called_endpoint(&self) -> Result<Vec<String>, Error> {
        let pipeline: Vec<Document> = vec![
            doc!{
                "$group": {
                    "_id": "$endpoint",
                    "count": { "$sum": 1 }
                }
            },
            doc!{
                "$sort": { "count": -1 }
            },
            doc!{
                "$group": {
                    "_id": null,
                    "maxCount": { "$max": "$count" },
                    "results": { "$push": { "endpoint": "$_id", "count": "$count" } }
                }
            },
            doc!{
                "$project": {
                    "results": {
                        "$filter": {
                            "input": "$results",
                            "as": "result",
                            "cond": { "$eq": ["$$result.count", "$maxCount"] }
                        }
                    },
                    "_id": 0
                }
            },
            doc!{
                "$limit": 1
            },
        ];
        
        let mut aggregation_cursor = self.col.aggregate(pipeline, None).await.ok().expect("Error aggregating analytics");
        let mut endpoints: Vec<String> = Vec::new();

        loop {
            match aggregation_cursor.try_next().await {
                Ok(Some(result)) => {
                    if let Ok(results) = bson::from_document::<MyDocument>(result.clone()) {
                        for result in results.results {
                            endpoints.push(result.endpoint);
                        }
                    } else {
                        println!("Error deserializing document into MyDocument");
                    }
                }
                Ok(None) => {
                    break;
                }
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
            }
        }
        

        Ok(endpoints)
    }
    pub async fn get_number_of_call_for_each_endpoint(&self) -> Result<Vec<CountEndpointsReponseDto>, Error> {
        let pipeline: Vec<Document> = vec![
            doc!{
                "$group": {
                    "_id": "$endpoint",
                    "count": { "$sum": 1 }
                }
            },
            doc!{
                "$sort": { "count": -1 }
            },
            doc!{
                "$group": {
                    "_id": null,
                    "results": { "$push": { "endpoint": "$_id", "count": "$count" } }
                }
            },
            doc!{
                "$limit": 1
            },
        ];
        
        let mut aggregation_cursor = self.col.aggregate(pipeline, None).await.ok().expect("Error aggregating analytics");
        let mut endpoints: Vec<CountEndpointsReponseDto> = Vec::new();

        loop {
            match aggregation_cursor.try_next().await {
                Ok(Some(result)) => {
                    if let Ok(results) = bson::from_document::<MyDocument>(result.clone()) {
                        for result in results.results {
                            endpoints.push(CountEndpointsReponseDto {
                                endpoint: result.endpoint,
                                count: result.count,
                            });
                        }
                    } else {
                        println!("Error deserializing document into MyDocument");
                    }
                }
                Ok(None) => {
                    break;
                }
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
            }
        }
        

        Ok(endpoints)
    }
}