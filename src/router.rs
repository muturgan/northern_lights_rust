use ::std::sync::Arc;

use axum::{routing::get, Router};
use tower_http::services::{ServeDir, ServeFile};

use crate::db::AppState;
use crate::handler::{favicon_handler, health_checker_handler};

pub fn create_router(app_state: Arc<AppState>) -> Router {
	return Router::new()
		.route("/api/healthchecker", get(health_checker_handler))
		.route("/favicon.ico", get(favicon_handler))
		.nest_service("/promo", ServeFile::new("static/promo.html"))
		.nest_service("/check", ServeFile::new("static/check.html"))
		.nest_service("/static", ServeDir::new("static"))
		.with_state(app_state);
}
