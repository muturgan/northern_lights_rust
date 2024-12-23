use axum::{
	body::Body,
	extract::Request,
	http::{HeaderValue, header},
	middleware::Next,
	response::{IntoResponse, Response},
};
use axum_extra::extract::cookie::CookieJar;

use crate::{
	config,
	system_models::{AppError, AppResponse},
};

pub async fn auth(cookie_jar: CookieJar, req: Request<Body>, next: Next) -> Response {
	let cookie_key = if config::is_test() {
		"authorization"
	} else {
		"__Secure-authorization"
	};

	let cookie_token = cookie_jar.get(cookie_key).map(|cookie| cookie.value());

	let token = cookie_token.or_else(|| {
		req.headers()
			.get(header::AUTHORIZATION)
			.and_then(|auth_header| auth_header.to_str().ok())
	});

	if let Some(pass) = token {
		if pass != config::get_admin_pass() {
			return AppResponse::from(AppError::unauthorized()).into_response();
		}
	} else {
		return AppResponse::from(AppError::unauthorized()).into_response();
	}

	let mut res = next.run(req).await;

	if cookie_token.is_none() {
		let secure = if config::is_test() { "" } else { "Secure; " };
		let auth_cookie = format!(
			"{cookie_key}={}; SameSite; {secure}HttpOnly",
			config::get_admin_pass()
		);
		match HeaderValue::from_str(&auth_cookie) {
			Err(_) => return AppError::system_error("Ошиюка установки cookie").into_response(),
			Ok(cookie_val) => {
				res.headers_mut().append(header::SET_COOKIE, cookie_val);
			}
		}
	}

	return res;
}
