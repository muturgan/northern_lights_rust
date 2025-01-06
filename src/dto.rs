use ::std::error::Error;
use axum::{
	Json, RequestExt,
	extract::{FromRequest, Request, rejection::JsonRejection},
};
use chrono::NaiveDate;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{
	Deserialize, Deserializer,
	de::{DeserializeOwned, Error as _},
};

use crate::system_models::AppError;

lazy_static! {
	static ref RE_PHONE: Regex =
		Regex::new(r"^(\+79)[0-9]{9}$").expect("Phone regex should build without errors");
	static ref RE_PROMO: Regex =
		Regex::new(r"^[а-я]{4,8}-[0-9]{3}$").expect("Promo regex should build without errors");
	static ref RE_DATE: Regex =
		Regex::new(r"^[0-9]{4}-[0-9]{2}-[0-9]{2}$").expect("Date regex should build without errors");
}

#[derive(Debug)]
pub struct RegistrationDto {
	pub phone: String,
	pub first_name: String,
	pub birth_date: NaiveDate,
}

impl<'de> Deserialize<'de> for RegistrationDto {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		#[derive(Deserialize)]
		struct PlainBody {
			phone: String,

			#[serde(rename = "firstName")]
			first_name: String,

			#[serde(rename = "birthDate")]
			birth_date: String,
		}

		let PlainBody {
			phone,
			first_name,
			birth_date,
		} = PlainBody::deserialize(deserializer)?;

		if !RE_PHONE.is_match(&phone) {
			return Err(D::Error::custom("Введён некорректный номер телефона"));
		}
		if first_name.is_empty() {
			return Err(D::Error::custom("Введено некорректное имя"));
		}
		if !RE_DATE.is_match(&birth_date) {
			return Err(D::Error::custom("Введена некорректная дата рождения"));
		}

		let birth_date = NaiveDate::parse_from_str(&birth_date, "%Y-%m-%d")
			.map_err(|_| D::Error::custom("Введена некорректная дата рождения"))?;

		Ok(Self {
			phone,
			first_name,
			birth_date,
		})
	}
}

#[derive(Debug)]
pub struct PromoDto {
	pub phone: String,
	pub promocode: String,
}

impl<'de> Deserialize<'de> for PromoDto {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		#[derive(Deserialize)]
		struct PlainBody {
			phone: String,
			promocode: String,
		}

		let PlainBody { phone, promocode } = PlainBody::deserialize(deserializer)?;

		if !RE_PHONE.is_match(&phone) {
			return Err(D::Error::custom("Введён некорректный номер телефона"));
		}
		if !RE_PROMO.is_match(&promocode) {
			return Err(D::Error::custom("Введён некорректный промокод"));
		}

		Ok(Self { phone, promocode })
	}
}

pub struct Dto<T>(pub T);

impl<T, S> FromRequest<S> for Dto<T>
where
	T: 'static + DeserializeOwned,
	S: Send + Sync,
{
	type Rejection = AppError;

	async fn from_request(req: Request, _: &S) -> Result<Self, Self::Rejection> {
		let body = req.extract::<Json<T>, _>().await;
		match body {
			Err(err) => Err(handle_json_rejection(err)),
			Ok(Json(dto)) => Ok(Dto(dto)),
		}
	}
}

fn handle_json_rejection(err: JsonRejection) -> AppError {
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
			AppError::system_error("Не удалось прочитать тело запроса")
		}

		non_exhaustive => AppError::system_error(non_exhaustive),
	};
}
