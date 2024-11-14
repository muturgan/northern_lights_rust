use ::std::error::Error;
use axum::{
	Json, RequestExt, async_trait,
	extract::{FromRequest, Request, rejection::JsonRejection},
};
use chrono::NaiveDate;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::system_models::AppError;

lazy_static! {
	static ref RE_PHONE: Regex =
		Regex::new(r"^(\+79)\d{9}$").expect("Phone regex should build without errors");
	static ref RE_PROMO: Regex =
		Regex::new(r"^[а-я]{4,8}-\d{3}$").expect("Promo regex should build without errors");
	static ref RE_DATE: Regex =
		Regex::new(r"^\d{4}-\d{2}-\d{2}$").expect("Date regex should build without errors");
}

#[derive(Serialize, Deserialize, Debug, Validate)]
struct RawRegistrationDto {
	#[validate(regex(path = "*RE_PHONE", message = "Введён некорректный номер телефона"))]
	pub phone: String,
	#[serde(rename = "firstName")]
	#[validate(length(min = 1, message = "Введено некорректное имя"))]
	pub first_name: String,
	#[serde(rename = "birthDate")]
	#[validate(regex(path = "*RE_DATE", message = "Введена некорректная дата рождения"))]
	pub birth_date: String,
}

#[derive(Debug)]
pub struct RegistrationDto {
	pub phone: String,
	pub first_name: String,
	pub birth_date: NaiveDate,
}

#[async_trait]
impl<S: Send + Sync> FromRequest<S> for RegistrationDto {
	type Rejection = AppError;

	async fn from_request(req: Request, _: &S) -> Result<Self, Self::Rejection> {
		let body = req.extract::<Json<RawRegistrationDto>, _>().await;
		let dto = handle_parsed_body(body)?;

		let birth_date = match NaiveDate::parse_from_str(&dto.birth_date, "%Y-%m-%d") {
			Err(_) => {
				return Err(AppError::ScenarioError(
					String::from("Введена некорректная дата рождения"),
					None,
				));
			}
			Ok(date) => date,
		};

		return Ok(RegistrationDto {
			phone: dto.phone,
			first_name: dto.first_name,
			birth_date,
		});
	}
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct PromoDto {
	#[validate(regex(path = "*RE_PHONE", message = "Введён некорректный номер телефона"))]
	pub phone: String,
	#[validate(regex(path = "*RE_PROMO", message = "Введён некорректный промокод"))]
	pub promocode: String,
}

#[async_trait]
impl<S: Send + Sync> FromRequest<S> for PromoDto {
	type Rejection = AppError;

	async fn from_request(req: Request, _: &S) -> Result<Self, Self::Rejection> {
		let body = req.extract::<Json<PromoDto>, _>().await;
		return handle_parsed_body(body);
	}
}

fn handle_json_rejection(err: &JsonRejection) -> AppError {
	return match err {
		JsonRejection::JsonDataError(data_err) => match data_err.source() {
			Some(source_err) => AppError::ScenarioError(
				format!("Передано некорректное тело запроса: {source_err}"),
				None,
			),
			None => AppError::ScenarioError(String::from("Передано некорректное тело запроса"), None),
		},

		JsonRejection::JsonSyntaxError(_) => {
			AppError::ScenarioError(String::from("Передано некорректное тело запроса"), None)
		}

		JsonRejection::MissingJsonContentType(_) => AppError::ScenarioError(
			String::from("Пожалуйста, укажите заголовок `Content-Type: application/json`"),
			None,
		),

		JsonRejection::BytesRejection(_) => {
			AppError::SystemError(String::from("Не удалось прочитать тело запроса"))
		}

		non_exhaustive => AppError::SystemError(non_exhaustive.to_string()),
	};
}

fn handle_parsed_body<T: Validate>(result: Result<Json<T>, JsonRejection>) -> Result<T, AppError> {
	return match result {
		Err(err) => Err(handle_json_rejection(&err)),
		Ok(Json(dto)) => match dto.validate() {
			Err(validation_error) => Err(AppError::ScenarioError(
				format!("Передано некорректное тело запроса: {validation_error}"),
				None,
			)),
			Ok(_) => Ok(dto),
		},
	};
}
