#![allow(clippy::needless_return)]

mod config;
mod db;
mod handler;
mod router;

use axum::Server;

#[tokio::main]
async fn main() {
	let app_state = db::create_db_connection().await;
	let app = router::create_router(app_state);

	let binded = Server::bind(&config::get_http_host_to_serve());
	println!(":) Server started successfully");
	binded.serve(app.into_make_service()).await.unwrap();
}
