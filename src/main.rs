//add the modules
mod api;
mod models;
mod repository;
mod utils;

use api::user_api::{create_user, delete_user, get_all_users, get_user, update_user};
use models::utils::ErrorResponse;
use repository::mongo_repo::MongoRepo;
use utils::helper::get_env_variable;
// use actix_web_validator::Json;
// use actix_web_validator::{Json, JsonConfig};

use actix_web::{
    error::InternalError,
    get,
    web::{Data, JsonConfig},
    App, HttpResponse, HttpServer, Responder,
};

use crate::utils::helper::get_timestamp;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello from rust and mongoDB")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = get_env_variable("PORT", "8080");
    let db = MongoRepo::init();
    let db_data = Data::new(db);
    let json_cfg = JsonConfig::default()
        // use custom error handler
        .error_handler(|err, req| {
            println!("{:#?} {:#?}", err, req);
            InternalError::from_response(
                err,
                HttpResponse::BadRequest().json(ErrorResponse::new(
                    "Please fill the required fields".to_string(),
                )),
            )
            .into()
        });

    let config = Data::new(json_cfg);
    // let configg = Data::new(query_config);

    println!("PORT {}", port);
    HttpServer::new(move || {
        get_timestamp();
        App::new()
            .app_data(config.clone())
            .app_data(db_data.clone())
            .service(create_user)
            .service(get_user)
            .service(get_all_users)
            .service(update_user)
            .service(delete_user)
    })
    .bind(("localhost", port.parse::<u16>().unwrap()))?
    .run()
    .await
}
