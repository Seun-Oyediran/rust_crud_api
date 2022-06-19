use std::env;
extern crate dotenv;
use crate::models::user_model::User;
use dotenv::dotenv;
use mongodb::sync::{Client, Collection};

pub struct MongoRepo {
    pub user_col: Collection<User>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");
        let user_col: Collection<User> = db.collection("User");
        MongoRepo { user_col }
    }
}
