use crate::auth::auth;
use crate::handler as H;
use crate::repository::Repository;
use ::std::sync::Arc;
use axum::{
	middleware,
	routing::{get, post},
	Router,
};
use tower_http::services::{ServeDir, ServeFile};

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
		.route("/", get(H::index_handler))
		.route("/favicon.ico", get(H::favicon_handler))
		.nest_service("/promo", ServeFile::new("static/promo.html"))
		.nest_service("/check", ServeFile::new("static/check.html"))
		.nest_service("/users", ServeFile::new("static/users.html"))
		.nest_service("/static", ServeDir::new("static"))
		.with_state(repo);
}
