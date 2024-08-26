use super::AppResponse;
use ::std::error::Error;
use ::std::fmt::{Display, Formatter, Result as FmtResult};
use axum::response::{IntoResponse, Response};

#[derive(Debug)]
pub enum AppError {
	UnauthorizedError(String),
	ScenarioError(String, Option<String>),
	SystemError(String),
}

impl Display for AppError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		return match self {
			AppError::UnauthorizedError(msg) => {
				write!(f, "UnauthorizedError: {}", msg)
			}
			AppError::ScenarioError(msg, ctx) => {
				let ctx_str = match ctx {
					Some(str) => format!(" ({str})"),
					None => String::default(),
				};
				write!(f, "ScenarioError: {msg}{ctx_str}")
			}
			AppError::SystemError(msg) => {
				write!(f, "SystemError: {}", msg)
			}
		};
	}
}

impl Error for AppError {}

impl AppError {
	pub fn unauthorized() -> Self {
		AppError::UnauthorizedError(String::from("Неверный пароль."))
	}

	pub fn user_already_exists(phone: String) -> Self {
		AppError::ScenarioError(
			format!("Пользователь с номером телефона {phone} уже существует"),
			Some(phone),
		)
	}

	pub fn promo_not_exists() -> Self {
		AppError::ScenarioError(String::from("Данный промокод не существует"), None)
	}

	pub fn promo_already_activated() -> Self {
		AppError::ScenarioError(
			String::from("Данный промокод уже был активирован ранее"),
			None,
		)
	}
}

impl IntoResponse for AppError {
	fn into_response(self) -> Response {
		AppResponse::from(self).into_response()
	}
}

impl<T> From<AppError> for Result<T, AppError> {
	fn from(err: AppError) -> Self {
		return Err(err);
	}
}
