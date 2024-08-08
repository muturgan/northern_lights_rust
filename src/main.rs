#![allow(clippy::needless_return)]

mod config;
mod db;
mod dto;
mod handler;
mod models;
mod router;
mod system_models;

#[tokio::main]
async fn main() {
	let app_state = db::create_db_connection().await;
	let app = router::create_router(app_state);

	let binded = tokio::net::TcpListener::bind(&config::get_http_host_to_serve())
		.await
		.unwrap();
	println!(":) Server started successfully");
	axum::serve(binded, app).await.unwrap();
}
