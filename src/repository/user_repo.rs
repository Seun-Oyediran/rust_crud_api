use super::mongo_repo::MongoRepo;
use crate::models::{user_model::User, utils::ErrorResponse};
use actix_web::HttpResponse;
use mongodb::{
    bson::{doc, oid::ObjectId},
    results::InsertOneResult,
};

impl MongoRepo {
    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, HttpResponse> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
            created_at: new_user.created_at,
            updated_at: new_user.updated_at,
        };

        let user = self.user_col.insert_one(new_doc, None);

        match user {
            Ok(result) => Ok(result),
            Err(_) => Err(HttpResponse::InternalServerError()
                .json(ErrorResponse::new("Server error".to_string()))),
        }
    }

    pub fn get_user(&self, id: &String) -> Result<User, HttpResponse> {
        let obj_id = match ObjectId::parse_str(&id) {
            Ok(data) => data,
            Err(_) => {
                return Err(
                    HttpResponse::BadRequest().json(ErrorResponse::new("Invalid id".to_string()))
                )
            }
        };
        let filter = doc! {"_id": obj_id};
        let user_detail = self.user_col.find_one(filter, None);
        match user_detail {
            Ok(Some(data)) => Ok(data),
            Ok(None) => {
                return Err(
                    HttpResponse::NotFound().json(ErrorResponse::new("User not found".to_string()))
                )
            }
            Err(_) => {
                return Err(HttpResponse::InternalServerError()
                    .json(ErrorResponse::new("Server error".to_string())))
            }
        }
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, HttpResponse> {
        // let new_doc = doc! {
        //      "$text": { "$search": "gunna POinter" } 
        // };
        let users = self.user_col.find(None, None);
        
        match users {
            Ok(data) => Ok(data.map(|doc| doc.unwrap()).collect()),
            Err(_) => Err(HttpResponse::InternalServerError()
                .json(ErrorResponse::new("Server error".to_string()))),
        }
    }

    pub fn update_user(&self, id: ObjectId, new_user: User) -> Result<User, HttpResponse> {
        let filter = doc! {"_id": id};
        let new_doc = doc! {
            "$set":
                {
                    "_id": new_user.id,
                    "name": new_user.name,
                    "location": new_user.location,
                    "title": new_user.title,
                    "updated_at": new_user.updated_at
                },
        };

        let updated_user = self.user_col.find_one_and_update(filter, new_doc, None);
        match updated_user {
            Ok(Some(user)) => Ok(user),
            Ok(None) => {
                Err(HttpResponse::NotFound().json(ErrorResponse::new("User not found".to_string())))
            }
            Err(_) => {
                return Err(HttpResponse::InternalServerError()
                    .json(ErrorResponse::new("Server error".to_string())))
            }
        }
    }

    pub fn delete_user(&self, id: ObjectId) -> Result<(), HttpResponse> {
        let filter = doc! {"_id": id};

        let result = self.user_col.find_one_and_delete(filter, None);
        match result {
            Ok(Some(_)) => Ok(()),
            Ok(None) => {
                Err(HttpResponse::NotFound().json(ErrorResponse::new("User not found".to_string())))
            }
            Err(_) => {
                return Err(HttpResponse::InternalServerError()
                    .json(ErrorResponse::new("Server error".to_string())))
            }
        }
    }
}
