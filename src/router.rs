use crate::handler::{favicon_handler, registration, users};
use crate::repository::Repository;
use ::std::sync::Arc;
use axum::{
	routing::{get, post},
	Router,
};
use tower_http::services::{ServeDir, ServeFile};

pub fn create_router(repo: Arc<Repository>) -> Router {
	return Router::new()
		.route("/api/registration", post(registration))
		.route("/api/users", get(users))
		.route("/favicon.ico", get(favicon_handler))
		.nest_service("/promo", ServeFile::new("static/promo.html"))
		.nest_service("/check", ServeFile::new("static/check.html"))
		.nest_service("/static", ServeDir::new("static"))
		.with_state(repo);
}
