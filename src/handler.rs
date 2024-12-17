use ::std::sync::Arc;
#[cfg(feature = "stream")]
use axum::{body::Body, http::header};
use axum::{
	extract::State,
	http::StatusCode,
	response::{IntoResponse, Redirect},
};
use lazy_static::lazy_static;
use rand::Rng;
#[cfg(feature = "stream")]
use tokio::{fs::File, io::BufReader};
#[cfg(feature = "stream")]
use tokio_util::io::ReaderStream;

use crate::{
	config,
	dto::{Dto, PromoDto, RegistrationDto},
	repository::Repository,
	system_models::{AppResponse, AppResult},
};

lazy_static! {
	static ref BIPS: Vec<String> = config::get_bips();
}

const MIN_POSTFIX_VALUE: u16 = 1;
const MAX_POSTFIX_VALUE: u16 = 999;
#[cfg(feature = "stream")]
const OCTET_STREAM: &str = "application/octet-stream";

pub async fn index_handler() -> Redirect {
	return Redirect::to("/promo");
}

pub async fn favicon_handler() -> impl IntoResponse {
	return StatusCode::NO_CONTENT;
}

pub async fn registration(
	State(repo): State<Arc<Repository>>,
	Dto(body): Dto<RegistrationDto>,
) -> AppResult {
	let promocode = generate_promo_from_bips();

	let query_result = repo
		.insert_user_and_grant_promo(&body.first_name, body.birth_date, &body.phone, &promocode)
		.await?;

	return AppResponse::user_registered(query_result.promocode);
}

pub async fn check(State(repo): State<Arc<Repository>>, Dto(body): Dto<PromoDto>) -> AppResult {
	repo.check_promo(&body.phone, &body.promocode).await?;
	return AppResponse::promo_valid();
}

pub async fn activate(State(repo): State<Arc<Repository>>, Dto(body): Dto<PromoDto>) -> AppResult {
	repo.activate_promo(&body.phone, &body.promocode).await?;
	return AppResponse::promo_activated();
}

pub async fn users(State(repo): State<Arc<Repository>>) -> AppResult {
	let users = repo.read_users().await?;
	return AppResponse::user_list(users);
}

#[cfg(feature = "stream")]
pub async fn read_bips() -> impl IntoResponse {
	let Ok(file) = File::open(config::get_bips_path()).await else {
		return StatusCode::INTERNAL_SERVER_ERROR.into_response();
	};

	let stream = ReaderStream::new(BufReader::new(file));
	let body = Body::from_stream(stream);

	([(header::CONTENT_TYPE, "text/plain")], body).into_response()
}

#[cfg(feature = "stream")]
pub async fn fetch() -> impl IntoResponse {
	let secure_postfix = if config::is_secure() { "s" } else { "" };
	let host = config::get_http_host_to_serve();
	let url = format!("http{secure_postfix}://{host}/api/bips");

	let Ok(res) = reqwest::get(url).await else {
		return StatusCode::INTERNAL_SERVER_ERROR.into_response();
	};

	let content_type = res.headers().get(header::CONTENT_TYPE);
	let content_type = content_type
		.map(|h| h.to_str().unwrap_or(OCTET_STREAM).to_string())
		.unwrap_or(OCTET_STREAM.into());

	let body = Body::from_stream(res.bytes_stream());

	([(header::CONTENT_TYPE, content_type)], body).into_response()
}

fn generate_promo_from_bips() -> String {
	return format!("{}-{}", generate_bip(), generate_postfix());
}

fn generate_bip() -> &'static str {
	let random_index = rand::thread_rng().gen_range(0..BIPS.len());
	return &BIPS[random_index];
}

fn generate_postfix() -> String {
	let random_int = rand::thread_rng().gen_range(MIN_POSTFIX_VALUE..MAX_POSTFIX_VALUE);
	return format!("{:>03}", random_int);
}
