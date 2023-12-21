use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Analytics {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub endpoint: String,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAnalyticsDto {
    pub endpoint: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CountEndpointsReponseDto {
    pub endpoint: String,
    pub count: i32,
}

#[derive(Debug, Deserialize)]
pub struct ResultItem {
    pub endpoint: String,
    pub count: i32,
}

#[derive(Debug, Deserialize)]
pub struct MyDocument {
    pub results: Vec<ResultItem>,
}