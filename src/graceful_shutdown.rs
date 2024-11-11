use ::std::sync::Arc;

use crate::repository::Repository;

pub async fn shutdown_signal(repo: Arc<Repository>) {
	let shutdown_fn = || async {
		repo.close().await;
	};

	let ctrl_c = || async {
		tokio::signal::ctrl_c()
			.await
			.expect("failed to install Ctrl+C handler");

		shutdown_fn().await;
	};

	#[cfg(unix)]
	let terminate = || async {
		tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
			.expect("failed to install signal handler")
			.recv()
			.await;

		shutdown_fn().await;
	};

	#[cfg(not(unix))]
	let terminate = std::future::pending::<()>();

	tokio::select! {
		_ = ctrl_c() => {},
		_ = terminate() => {},
	}
}
