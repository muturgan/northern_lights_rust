use ::std::sync::Arc;
use axum::{
	extract::State,
	http::StatusCode,
	response::{IntoResponse, Redirect},
};
use lazy_static::lazy_static;
use rand::Rng;

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
