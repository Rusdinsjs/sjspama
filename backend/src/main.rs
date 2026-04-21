pub mod models;
pub mod handlers;

use axum::{routing::{get, post, delete}, Router};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use argon2::password_hash::PasswordHasher;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = dotenvy::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://root:rootpassword@localhost:5432/sjspama_dev".to_string());

    tracing::info!("Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to Postgres");

    tracing::info!("Running migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    // Ensure seed admin user exists
    {
        tracing::info!("Ensuring seed admin user...");
        use argon2::password_hash::rand_core::OsRng;
        let salt = argon2::password_hash::SaltString::generate(&mut OsRng);
        let argon2 = argon2::Argon2::default();
        let password_hash = argon2
            .hash_password("admin123".as_bytes(), &salt)
            .unwrap()
            .to_string();

        sqlx::query(
            "INSERT INTO users (name, email, password_hash, role) 
             VALUES ($1, $2, $3, $4)
             ON CONFLICT (email) DO UPDATE 
             SET password_hash = EXCLUDED.password_hash"
        )
        .bind("System Admin")
        .bind("admin@sjs.id")
        .bind(password_hash)
        .bind("ADMIN")
        .execute(&pool)
        .await
        .unwrap();
        tracing::info!("Seed admin is ready: admin@sjs.id / admin123");
    }

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/health", get(|| async { "SJS-PAMA Backend OK" }))
        .route("/api/units", get(handlers::get_units).post(handlers::register_unit))
        .route("/api/units/:id", delete(handlers::delete_unit).put(handlers::update_unit))
        .route("/api/units/:id/logs", get(handlers::get_unit_logs))
        .route("/api/daily-logs", post(handlers::create_daily_log))
        .route("/api/employees", get(handlers::get_employees).post(handlers::register_employee))
        .route("/api/employees/:id", delete(handlers::delete_employee).put(handlers::update_employee))
        .route("/api/auth/login", post(handlers::login))
        .route("/api/users", get(handlers::get_all_users).post(handlers::register_user))
        .route("/api/users/:id", delete(handlers::delete_user).put(handlers::update_user))
        .route("/api/users/:id/profile", axum::routing::put(handlers::update_profile))
        .route("/api/work-locations", get(handlers::get_work_locations).post(handlers::create_work_location))
        .route("/api/work-locations/:id", delete(handlers::delete_work_location).put(handlers::update_work_location))
        .route("/api/assignments", get(handlers::get_operator_assignments).post(handlers::create_operator_assignment))
        .route("/api/assignments/:id", delete(handlers::delete_operator_assignment))
        .route("/api/positions", get(handlers::get_positions).post(handlers::create_position))
        .route("/api/positions/:id", delete(handlers::delete_position))
        .route("/api/licenses", get(handlers::get_licenses).post(handlers::create_license))
        .route("/api/licenses/:id", delete(handlers::delete_license).put(handlers::update_license))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(cors)
        .with_state(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8081));
    tracing::info!("Server listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
