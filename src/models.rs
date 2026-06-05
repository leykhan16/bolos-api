use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc, NaiveDate};

#[derive(Debug, Serialize, FromRow)]
pub struct Service {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price_from: f64,
    pub duration_hrs: i32,
    pub icon: String,
    pub popular: bool,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Booking {
    pub id: i32,
    pub full_name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub service_id: i32,
    pub service_name: String,
    pub preferred_date: NaiveDate,
    pub preferred_time: String,
    pub notes: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateBookingRequest {
    pub full_name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub service_id: i32,
    pub preferred_date: NaiveDate,
    pub preferred_time: String,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBookingStatus {
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct ContactMessage {
    pub full_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub subject: String,
    pub message: String,
}