use crate::config;
use crate::dto::RegistrationDto;
use crate::repository::Repository;
use crate::system_models::ApiResponse;
use ::std::sync::Arc;
use axum::{
	extract::State,
	http::StatusCode,
	response::{IntoResponse, Redirect},
	Json,
};
use chrono::NaiveDate;
use pad::{Alignment, PadStr};
use rand::Rng;

const MIN_POSTFIX_VALUE: usize = 1;
const MAX_POSTFIX_VALUE: usize = 999;
const MAX_POSTFIX_LENGTH: usize = 3;

pub async fn index_handler() -> Redirect {
	return Redirect::to("/promo");
}

pub async fn favicon_handler() -> impl IntoResponse {
	return StatusCode::NO_CONTENT;
}

pub async fn registration(
	State(repo): State<Arc<Repository>>,
	Json(body): Json<RegistrationDto>,
) -> ApiResponse {
	let birth_date = NaiveDate::parse_from_str(&body.birth_date, "%Y-%m-%d").unwrap();

	let promocode = generate_promo_from_bips();

	let query_result = repo
		.insert_user_and_grant_promo(body.firstName, birth_date, body.phone, promocode)
		.await;

	return match query_result {
		Err(err) => ApiResponse::from(err),
		Ok(p) => ApiResponse::user_registered(p.promocode),
	};
}

pub async fn users(State(repo): State<Arc<Repository>>) -> ApiResponse {
	let query_result = repo.read_users().await;
	return match query_result {
		Err(err) => ApiResponse::from(err),
		Ok(users) => ApiResponse::user_list(users),
	};
}

fn generate_promo_from_bips() -> String {
	let mut promocode = generate_bip();
	promocode.push('-');
	let postfix = generate_postfix();
	promocode.push_str(&postfix);
	return promocode;
}

fn generate_bip() -> String {
	let bips = config::get_bips();
	let random_index = rand::thread_rng().gen_range(0..bips.len());
	let bip = &bips[random_index];
	return String::from(bip);
}

fn generate_postfix() -> String {
	let random_int = rand::thread_rng().gen_range(MIN_POSTFIX_VALUE..MAX_POSTFIX_VALUE);
	return random_int
		.to_string()
		.pad(MAX_POSTFIX_LENGTH, '0', Alignment::Right, true);
}
