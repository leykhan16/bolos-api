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
pub async fn get_messages(pool: web::Data<PgPool>) -> HttpResponse {
    let result = sqlx::query!(
        "SELECT id, full_name, email, phone, subject, message FROM contact_messages ORDER BY id DESC"
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(rows) => {
            let messages: Vec<serde_json::Value> = rows.iter().map(|r| {
                serde_json::json!({
                    "id": r.id,
                    "full_name": r.full_name,
                    "email": r.email,
                    "phone": r.phone,
                    "subject": r.subject,
                    "message": r.message,
                })
            }).collect();
            HttpResponse::Ok().json(messages)
        }
        Err(e) => {
            eprintln!("Failed to fetch messages: {e}");
            HttpResponse::InternalServerError().body("Failed to fetch messages")
        }
    }
}
