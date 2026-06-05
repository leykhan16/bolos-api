use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use crate::models::ContactMessage;

pub async fn send_message(
    pool: web::Data<PgPool>,
    body: web::Json<ContactMessage>,
) -> HttpResponse {
    let result = sqlx::query!(
        r#"
        INSERT INTO contact_messages (full_name, email, phone, subject, message)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id
        "#,
        body.full_name,
        body.email,
        body.phone,
        body.subject,
        body.message,
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Message received! We'll get back to you within 24 hours."),
        Err(e) => {
            eprintln!("Failed to save message: {e}");
            HttpResponse::InternalServerError().body("Failed to send message")
        }
    }
}