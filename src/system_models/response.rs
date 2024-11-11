use axum::{
	Json,
	http::StatusCode,
	response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

use super::AppResult;
use crate::{
	repository::models,
	system_models::{errors::AppError, scenario_status::EScenarioStatus},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct AppResponse {
	pub status: EScenarioStatus,
	pub result: String,
	pub payload: Option<serde_json::Value>,
}

impl AppResponse {
	fn new(status: EScenarioStatus, result: String, payload: Option<serde_json::Value>) -> Self {
		return Self {
			status,
			result,
			payload,
		};
	}

	fn scenario_success<S: AsRef<str>>(result: S, payload: Option<serde_json::Value>) -> Self {
		return Self::new(
			EScenarioStatus::SCENARIO_SUCCESS,
			result.as_ref().to_string(),
			payload,
		);
	}

	pub fn unauthorized(result: String, payload: Option<serde_json::Value>) -> Self {
		return Self::new(EScenarioStatus::UNAUTHORIZED, result, payload);
	}

	fn scenario_fail(result: String, payload: Option<serde_json::Value>) -> Self {
		return Self::new(EScenarioStatus::SCENARIO_FAIL, result, payload);
	}

	pub fn system_error(result: String, payload: Option<serde_json::Value>) -> Self {
		return Self::new(EScenarioStatus::SYSTEM_ERROR, result, payload);
	}

	//  *********************************
	//  *                               *
	//  *       Scenario Success        *
	//  *                               *
	//  *********************************

	pub fn user_registered(promocode: String) -> AppResult {
		let upper = promocode.to_uppercase();
		return Ok(Self::scenario_success(
			format!("Новый пользователь успешно зарегистрирован. Промокод: {upper}"),
			None,
		));
	}

	pub fn promo_valid() -> AppResult {
		return Ok(Self::scenario_success("Промокод корректен", None));
	}

	pub fn promo_activated() -> AppResult {
		return Ok(Self::scenario_success("Промокод успешно активирован", None));
	}

	pub fn user_list(users: Vec<models::RegisteredUser>) -> AppResult {
		let payload = serde_json::json!(users);
		return Ok(Self::scenario_success(
			"Список пользователей",
			Some(payload),
		));
	}
}

impl IntoResponse for AppResponse {
	fn into_response(self) -> Response {
		(StatusCode::OK, Json(self)).into_response()
	}
}

impl From<AppError> for AppResponse {
	fn from(err: AppError) -> Self {
		match err {
			AppError::UnauthorizedError(result) => AppResponse::unauthorized(result, None),
			AppError::ScenarioError(result, payload) => {
				AppResponse::scenario_fail(result, payload.map(|p| serde_json::json!(p)))
			}
			AppError::SystemError(result) => AppResponse::system_error(result, None),
		}
	}
}
