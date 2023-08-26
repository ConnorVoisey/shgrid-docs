pub mod routes;
pub mod models {
    pub mod organisation;
    pub mod contact;
}

use dotenvy::dotenv;
use routes::create_routes;
use sqlx::{postgres::PgPoolOptions, Pool};

pub async fn connect() -> Pool<sqlx::Postgres> {
    dotenv().ok();
    let uri_string =
        dotenvy::var("DATABASE_URL").expect("Failed to read env variable 'DATABASE_URL'");

    // run_migrations(&pool)
    //     .await
    //     .expect("DB state did not match migrations");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&uri_string)
        .await
        .expect("Failed to connect to database at provided uri")
}

// async fn run_migrations(pool: &Pool<sqlx::Postgres>) -> Result<(), MigrateError> {
//     sqlx::migrate!().run(pool).await
// }

pub async fn run() {
    let app = create_routes(connect().await);
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
