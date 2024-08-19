use crate::repository::Repository;
use ::std::sync::Arc;

pub async fn shutdown_signal(repo: Arc<Repository>) {
	let ctrl_c = || async {
		tokio::signal::ctrl_c()
			.await
			.expect("failed to install Ctrl+C handler");

		repo.close().await;
	};

	#[cfg(unix)]
	let terminate = || async {
		tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
			.expect("failed to install signal handler")
			.recv()
			.await;

		repo.close().await;
	};

	#[cfg(not(unix))]
	let terminate = std::future::pending::<()>();

	tokio::select! {
		_ = ctrl_c() => {},
		_ = terminate() => {},
	}
}
