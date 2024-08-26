use crate::config;
use crate::system_models::{AppError, AppResponse};
use axum::{
	body::Body,
	extract::Request,
	http::{header, HeaderValue},
	middleware::Next,
	response::{IntoResponse, Response},
};
use axum_extra::extract::cookie::CookieJar;

pub async fn auth(cookie_jar: CookieJar, req: Request<Body>, next: Next) -> Response {
	let cookie_key = if config::is_test() {
		"authorization"
	} else {
		"__Secure-authorization"
	};

	let cookie_token = cookie_jar.get(cookie_key).map(|cookie| cookie.value());

	let token = if cookie_token.is_some() {
		cookie_token
	} else {
		req.headers()
			.get(header::AUTHORIZATION)
			.and_then(|auth_header| match auth_header.to_str() {
				Err(_) => None,
				Ok(val) => Some(val),
			})
	};

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

	let mut res = next.run(req).await;

	if cookie_token.is_none() {
		let secure = if config::is_test() { "" } else { "Secure; " };
		let auth_cookie = format!(
			"{cookie_key}={}; SameSite; {secure}HttpOnly",
			config::get_admin_pass()
		);
		res.headers_mut().append(
			header::SET_COOKIE,
			HeaderValue::from_str(&auth_cookie).unwrap(),
		);
	}

	return res;
}
