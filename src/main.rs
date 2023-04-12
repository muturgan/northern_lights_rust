#![allow(clippy::needless_return)]

mod config;

use ::std::collections::HashMap;
use ::std::sync::Arc;
use axum::{
	extract::Path, http::StatusCode, response::IntoResponse, routing::get, Json, Router, Server,
};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tower_http::services::{ServeDir, ServeFile};

const MESSAGE: &str = "Simple CRUD API with Rust, SQLX, Postgres,and Axum";

// registration
// check
// activate
// users

async fn health_checker_handler() -> impl IntoResponse {
	let json_response = serde_json::json!({
		"status": "success",
		"message": MESSAGE
	});

	return Json(json_response);
}

pub async fn index_handler(Path(params): Path<HashMap<String, String>>) -> impl IntoResponse {
	println!("params: {:?}", params);
	return "<p>Hello, World!</p>";
}

async fn favicon_handler() -> impl IntoResponse {
	return StatusCode::NO_CONTENT;
}

pub struct AppState {
	pub db: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
	let database_url = config::get_db_config();
	let pool = PgPoolOptions::new()
		.max_connections(config::get_db_max_pool_size())
		.connect(&database_url)
		.await
		.expect(":( Failed to connect to the database");
	println!(":) Connection to the database is successful");

	sqlx::migrate!("./migrations")
		.run(&pool)
		.await
		.expect(":( Migrations failed");
	println!(":) Migrations finished");

	let app_state = Arc::new(AppState { db: pool });

	let app = Router::new()
		.route("/api/healthchecker", get(health_checker_handler))
		.route("/favicon.ico", get(favicon_handler))
		.nest_service("/promo", ServeFile::new("static/promo.html"))
		.nest_service("/check", ServeFile::new("static/check.html"))
		.nest_service("/static", ServeDir::new("static"))
		.with_state(app_state);

	let binded = Server::bind(&config::get_http_host_to_serve());
	println!(":) Server started successfully");
	binded.serve(app.into_make_service()).await.unwrap();
}
