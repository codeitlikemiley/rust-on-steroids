mod config;
mod errors;

use crate::errors::CustomError;
use axum::{extract::Extension, response::Json, routing::get, Router};

use tokio::net::TcpListener;
use db::User;
use log::info;

#[tokio::main]
async fn main() {
    let config = config::Config::new();

    let pool = db::create_pool(&config.database_url);

    // build our application with a route
    let app = Router::new()
        .route("/", get(users))
        .layer(Extension(config))
        .layer(Extension(pool.clone()));

    // run it
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    info!("{:<12} - {:?}\n", "Listening on {}", listener.local_addr());

    axum::serve(listener,app.into_make_service())
        .await
        .unwrap();
}

async fn users(Extension(pool): Extension<db::Pool>) -> Result<Json<Vec<User>>, CustomError> {
    let client = pool.get().await?;

    let users = db::queries::users::get_users()
        .bind(&client)
        .all()
        .await?;

    Ok(Json(users))
}