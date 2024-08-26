use crate::config;
use crate::system_models::{AppError, AppResponse};
use axum::{
	body::Body,
	extract::Request,
	http::header,
	middleware::Next,
	response::{IntoResponse, Response},
};
use axum_extra::extract::cookie::CookieJar;

pub async fn auth(cookie_jar: CookieJar, req: Request<Body>, next: Next) -> Response {
	let token = cookie_jar
		.get("__Secure-authorization")
		.map(|cookie| cookie.value())
		.or_else(|| {
			req.headers()
				.get(header::AUTHORIZATION)
				.and_then(|auth_header| match auth_header.to_str() {
					Err(_) => None,
					Ok(val) => Some(val),
				})
		});

	match token {
		None => {
			return AppResponse::from(AppError::unauthorized()).into_response();
		}
		Some(pass) => {
			if pass != config::get_admin_pass() {
				return AppResponse::from(AppError::unauthorized()).into_response();
			}
		}
	}

	return next.run(req).await;
}
