use ::std::{error::Error, sync::Arc};
use promo_codes::{config, graceful_shutdown::shutdown_signal, repository::Repository, router};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let repo = Repository::new().await;
	let repo = Arc::new(repo);
	let app = router::create_router(repo.clone());

	let addr = config::get_http_host_to_serve();
	let listener = tokio::net::TcpListener::bind(addr).await?;

	println!(":) Server started successfully");

	axum::serve(listener, app)
		.with_graceful_shutdown(shutdown_signal(repo))
		.await?;

	Ok(())
}
