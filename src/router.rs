use ::std::sync::Arc;
use axum::{
	Router, middleware,
	routing::{get, post},
};
use tower_http::services::{ServeDir, ServeFile};

use crate::{auth::auth, handler as H, repository::Repository};

pub fn create_router(repo: Arc<Repository>) -> Router {
	return Router::new()
		.route("/api/registration", post(H::registration))
		.route(
			"/api/check",
			post(H::check).route_layer(middleware::from_fn(auth)),
		)
		.route(
			"/api/activate",
			post(H::activate).route_layer(middleware::from_fn(auth)),
		)
		.route("/api/users", get(H::users))
		.route("/api/bips", get(H::read_bips))
		.route("/", get(H::index_handler))
		.route("/favicon.ico", get(H::favicon_handler))
		.nest_service("/promo", ServeFile::new("static/promo.html"))
		.nest_service("/check", ServeFile::new("static/check.html"))
		.nest_service("/users", ServeFile::new("static/users.html"))
		.nest_service("/static", ServeDir::new("static"))
		.with_state(repo);
}
