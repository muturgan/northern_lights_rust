use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use chrono::NaiveDate;
use pad::{Alignment, PadStr};
use rand::Rng;
use sqlx::PgPool;

use crate::config;
use crate::dto::RegistrationDto;
use crate::models::{InsertedPromo, InsertedUser, User};
use crate::system_models::api_response::ApiResponse;

const MIN_POSTFIX_VALUE: usize = 1;
const MAX_POSTFIX_VALUE: usize = 999;
const MAX_POSTFIX_LENGTH: usize = 3;

pub async fn favicon_handler() -> impl IntoResponse {
	return StatusCode::NO_CONTENT;
}

pub async fn registration(
	Extension(pool): Extension<PgPool>,
	Json(body): Json<RegistrationDto>,
) -> ApiResponse {
	println!("body #1: {:?}", body);

	let query_result = sqlx::query_as::<_, InsertedUser>(
		"INSERT INTO users (firstname,birthdate,phone) VALUES ($1, $2, $3) RETURNING id",
	)
	.bind(body.firstName)
	.bind(NaiveDate::parse_from_str(&body.birth_date, "%Y-%m-%d").unwrap())
	.bind(&body.phone)
	.fetch_one(&pool)
	.await;

	let inserted_user = match query_result {
		Err(err) => {
			let err_str = err.to_string();
			if err_str.contains("duplicate key") {
				return ApiResponse::user_already_exists(body.phone);
			} else {
				return ApiResponse::system_error(err_str, None);
			}
		}
		Ok(u) => u,
	};

	let promocode = generate_promo_from_bips();

	let query_result = sqlx::query_as::<_, InsertedPromo>(
		"INSERT INTO promo (promocode,holder_id) VALUES ($1, $2) RETURNING promocode",
	)
	.bind(promocode)
	.bind(inserted_user.id)
	.fetch_one(&pool)
	.await;

	let inserted_promo = match query_result {
		Err(err) => {
			return ApiResponse::system_error(err.to_string(), None);
		}
		Ok(p) => p,
	};

	return ApiResponse::user_registered(inserted_promo.promocode);
}

pub async fn users(Extension(pool): Extension<PgPool>) -> ApiResponse {
	let query_result = sqlx::query_as::<_, User>("SELECT * FROM users")
		.fetch_all(&pool)
		.await;

	let users = match query_result {
		Err(err) => {
			return ApiResponse::system_error(err.to_string(), None);
		}
		Ok(u) => u,
	};

	return ApiResponse::user_list(users);
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
	println!("random_int: {random_int}");
	return random_int
		.to_string()
		.pad(MAX_POSTFIX_LENGTH, '0', Alignment::Right, true);
}
