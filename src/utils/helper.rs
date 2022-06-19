extern crate dotenv;
use super::super::models::utils::ErrorResponse;
use actix_web::HttpResponse;
use dotenv::dotenv;
use mongodb::bson::oid::ObjectId;
use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

pub fn get_env_variable(key: &str, fallback_value: &str) -> String {
    dotenv().ok();
    let value = match env::var(key) {
        Ok(value) => value,
        Err(_) => {
            println!(
                "Could not find variable, using fallback variable {}",
                fallback_value
            );
            fallback_value.to_string()
        }
    };
    value
}

pub fn get_timestamp() -> f64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let time = since_the_epoch.as_secs_f64();
    time * 1000.0
}

pub fn is_id_valid(id: String) -> Result<ObjectId, HttpResponse> {
    if id.is_empty() {
        return Err(HttpResponse::BadRequest().json(ErrorResponse::new("Invalid id".to_string())));
    }

    let obj_id = match ObjectId::parse_str(&id) {
        Ok(data) => data,
        Err(_) => {
            return Err(
                HttpResponse::BadRequest().json(ErrorResponse::new("Invalid id".to_string()))
            )
        }
    };

    Ok(obj_id)
}
