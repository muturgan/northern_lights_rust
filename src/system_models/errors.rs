use ::std::error::Error;
use ::std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub enum AppError {
	ScenarioError(String, Option<String>),
	SystemError(String),
}

impl Display for AppError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		return match self {
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
		AppError::ScenarioError(String::from("Данный промокод уже был активирован"), None)
	}
}
