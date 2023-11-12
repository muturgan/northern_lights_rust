use crate::models;
use crate::system_models::scenario_status::EScenarioStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponse {
	status: EScenarioStatus,
	result: String,
	payload: Option<serde_json::Value>,
}

impl ApiResponse {
	fn new(status: EScenarioStatus, result: String, payload: Option<serde_json::Value>) -> Self {
		return Self {
			status,
			result,
			payload,
		};
	}

	fn scenario_success(result: String, payload: Option<serde_json::Value>) -> Self {
		return Self::new(EScenarioStatus::SCENARIO_SUCCESS, result, payload);
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

	pub fn user_registered(promocode: String) -> Self {
		let upper = promocode.to_uppercase();
		return Self::scenario_success(
			format!("Новый пользователь успешно зарегистрирован. Промокод: {upper}"),
			None,
		);
	}

	pub fn promo_valid() -> Self {
		return Self::scenario_success(String::from("Промокод корректен"), None);
	}

	pub fn promo_activated() -> Self {
		return Self::scenario_success(String::from("Промокод успешно активирован"), None);
	}

	pub fn user_list(users: Vec<models::User>) -> Self {
		let payload = serde_json::json!(users);
		return Self::scenario_success(String::from("Список пользователей"), Some(payload));
	}

	//  *********************************
	//  *                               *
	//  *        Scenario Fail          *
	//  *                               *
	//  *********************************

	pub fn user_already_exists(phone: String) -> Self {
		return Self::scenario_fail(
			format!("Пользователь с номером телефона {phone} уже существует"),
			Some(serde_json::json!(phone)),
		);
	}

	pub fn promo_not_exists() -> Self {
		return Self::scenario_fail(String::from("Данный промокод не существует"), None);
	}

	pub fn promo_already_activated() -> Self {
		return Self::scenario_fail(String::from("Данный промокод уже был активирован"), None);
	}
}
