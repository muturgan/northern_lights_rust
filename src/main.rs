#![allow(clippy::needless_return)]

mod config;
mod db;
mod dto;
mod handler;
mod models;
mod repo;
mod router;
mod system_models;

#[tokio::main]
async fn main() {
	let app_state = db::create_db_connection().await;
	let app = router::create_router(app_state);

	async fn shutdown_signal() {
		let ctrl_c = async {
			tokio::signal::ctrl_c()
				.await
				.expect("failed to install Ctrl+C handler");
		};

		#[cfg(unix)]
		let terminate = async {
			tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
				.expect("failed to install signal handler")
				.recv()
				.await;
		};

		#[cfg(not(unix))]
		let terminate = std::future::pending::<()>();

		tokio::select! {
			_ = ctrl_c => {},
			_ = terminate => {},
		}
	}

	let listener = tokio::net::TcpListener::bind(&config::get_http_host_to_serve())
		.await
		.unwrap();
	println!(":) Server started successfully");
	axum::serve(listener, app)
		.with_graceful_shutdown(shutdown_signal())
		.await
		.unwrap();
}
