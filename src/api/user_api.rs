use crate::{
    models::{
        user_model::User,
        utils::{ErrorResponse, SuccessResponse},
    },
    repository::mongo_repo::MongoRepo,
    utils::helper::get_timestamp,
};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse, Scope,
};
// use actix_web_validator::Json;
use mongodb::bson::oid::ObjectId;

pub fn user_controller() -> Scope {
    let user_controller = actix_web::web::scope("")
        .service(create_user)
        .service(get_user)
        .service(get_all_users)
        .service(update_user)
        .service(delete_user);

    user_controller
}

#[post("/user")]
async fn create_user(db: Data<MongoRepo>, new_user: Json<User>) -> HttpResponse {
    let data = User {
        id: None,
        name: new_user.name.to_owned(),
        location: new_user.location.to_owned(),
        title: new_user.title.to_owned(),
        created_at: Some(get_timestamp()),
        updated_at: Some(get_timestamp()),
    };

    let user_detail = db.create_user(data);
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => err,
    }
}

#[get("/user/{id}")]
async fn get_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().json(ErrorResponse::new("Invalid id".to_string()));
    }

    let user_detail = db.get_user(&id);
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(SuccessResponse::new(user, "User found".to_string())),
        Err(err) => err,
    }
}

#[get("/user")]
async fn get_all_users(db: Data<MongoRepo>) -> HttpResponse {
    let data = db.get_all_users();
    match data {
        Ok(users) => HttpResponse::Ok().json(SuccessResponse::new(users, "User found".to_string())),
        Err(err) => err,
    }
}

#[put("/user/{id}")]
async fn update_user(
    db: Data<MongoRepo>,
    path: Path<String>,
    new_user: Json<User>,
) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().json(ErrorResponse::new("Invalid id".to_string()));
    }

    let obj_id = match ObjectId::parse_str(&id) {
        Ok(data) => data,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse::new("Invalid id".to_string()))
        }
    };

    let data = User {
        id: Some(obj_id),
        location: new_user.location.to_owned(),
        name: new_user.name.to_owned(),
        title: new_user.title.to_owned(),
        created_at: None,
        updated_at: Some(get_timestamp()),
    };

    let data = db.update_user(obj_id, data);
    match data {
        Ok(users) => HttpResponse::Ok().json(SuccessResponse::new(
            users,
            "User updated successfully".to_string(),
        )),
        Err(err) => err,
    }
}

#[delete("/user/{id}")]
async fn delete_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().json(ErrorResponse::new("Invalid id".to_string()));
    }

    let ans = db.delete_user(&id);
    match ans {
        Ok(users) => HttpResponse::Ok().json(SuccessResponse::new(
            users,
            "Deleted successfully".to_string(),
        )),
        Err(err) => err,
    }
}
