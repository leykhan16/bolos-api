use actix_web::{web, App, HttpServer, HttpResponse};
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
use actix_cors::Cors;

mod models;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    println!("✅ Database connected");

    let pool = web::Data::new(pool);

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_addr = format!("0.0.0.0:{port}");

    println!("🚀 Bolos API running on http://{bind_addr}");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(pool.clone())
            .route("/health", web::get().to(health_check))
            .route("/api/services", web::get().to(handlers::services::get_services))
            .route("/api/bookings", web::post().to(handlers::bookings::create_booking))
            .route("/api/bookings", web::get().to(handlers::bookings::get_bookings))
            .route("/api/bookings/{id}/status", web::patch().to(handlers::bookings::update_booking_status))
            .route("/api/contact", web::post().to(handlers::contact::send_message))
            .route("/api/contact", web::get().to(handlers::contact::get_messages))
    })
    .bind(&bind_addr)?
    .run()
    .await
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body("Bolos Fumigation API is alive!")
}