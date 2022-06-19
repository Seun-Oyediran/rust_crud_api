use actix_web::HttpResponse;

use crate::models::utils::ErrorResponse;

pub async fn route_not_found() -> HttpResponse {
    HttpResponse::NotFound().json(ErrorResponse::new("Resource not found".to_string()))
}
