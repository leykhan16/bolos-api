use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use crate::models::{Booking, CreateBookingRequest, UpdateBookingStatus};

pub async fn create_booking(
    pool: web::Data<PgPool>,
    body: web::Json<CreateBookingRequest>,
) -> HttpResponse {
    // First check the service exists and get its name
    let service = sqlx::query!(
        "SELECT name FROM services WHERE id = $1",
        body.service_id
    )
    .fetch_optional(pool.get_ref())
    .await;

    let service_name = match service {
        Ok(Some(row)) => row.name,
        Ok(None) => return HttpResponse::BadRequest().body("Invalid service ID"),
        Err(e) => {
            eprintln!("DB error: {e}");
            return HttpResponse::InternalServerError().body("Database error");
        }
    };

    // Insert the booking
    let result = sqlx::query_as!(
        Booking,
        r#"
        INSERT INTO bookings
            (full_name, email, phone, address, service_id, service_name,
             preferred_date, preferred_time, notes, status)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 'pending')
        RETURNING *
        "#,
        body.full_name,
        body.email,
        body.phone,
        body.address,
        body.service_id,
        service_name,
        body.preferred_date,
        body.preferred_time,
        body.notes,
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(booking) => HttpResponse::Created().json(booking),
        Err(e) => {
            eprintln!("Failed to create booking: {e}");
            HttpResponse::InternalServerError().body("Failed to create booking")
        }
    }
}

pub async fn get_bookings(pool: web::Data<PgPool>) -> HttpResponse {
    let result = sqlx::query_as!(
        Booking,
        "SELECT * FROM bookings ORDER BY created_at DESC"
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(bookings) => HttpResponse::Ok().json(bookings),
        Err(e) => {
            eprintln!("Failed to fetch bookings: {e}");
            HttpResponse::InternalServerError().body("Failed to fetch bookings")
        }
    }
}

pub async fn update_booking_status(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    body: web::Json<UpdateBookingStatus>,
) -> HttpResponse {
    let valid = ["pending", "confirmed", "completed", "cancelled"];
    if !valid.contains(&body.status.as_str()) {
        return HttpResponse::BadRequest().body("Invalid status");
    }

    let result = sqlx::query_as!(
        Booking,
        "UPDATE bookings SET status = $1 WHERE id = $2 RETURNING *",
        body.status,
        path.into_inner()
    )
    .fetch_optional(pool.get_ref())
    .await;

    match result {
        Ok(Some(booking)) => HttpResponse::Ok().json(booking),
        Ok(None) => HttpResponse::NotFound().body("Booking not found"),
        Err(e) => {
            eprintln!("Failed to update status: {e}");
            HttpResponse::InternalServerError().body("Failed to update status")
        }
    }
}