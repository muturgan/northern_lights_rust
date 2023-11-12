use axum::{
	routing::{get, post},
	Extension, Router,
};
use sqlx::PgPool;
use tower_http::services::{ServeDir, ServeFile};

use crate::handler::{favicon_handler, registration, users};

pub fn create_router(db: PgPool) -> Router {
	return Router::new()
		.route("/api/registration", post(registration))
		.route("/api/users", get(users))
		.route("/favicon.ico", get(favicon_handler))
		.nest_service("/promo", ServeFile::new("static/promo.html"))
		.nest_service("/check", ServeFile::new("static/check.html"))
		.nest_service("/static", ServeDir::new("static"))
		.layer(Extension(db));
}
