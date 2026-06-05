use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use crate::models::Service;

pub async fn get_services(pool: web::Data<PgPool>) -> HttpResponse {
    let result = sqlx::query_as!(
        Service,
        "SELECT * FROM services ORDER BY id ASC"
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(services) => HttpResponse::Ok().json(services),
        Err(e) => {
            eprintln!("Failed to fetch services: {e}");
            HttpResponse::InternalServerError().body("Failed to fetch services")
        }
    }
}