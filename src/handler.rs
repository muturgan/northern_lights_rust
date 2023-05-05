// use ::std::collections::HashMap;
// use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use axum::{http::StatusCode, response::IntoResponse, Json};

const MESSAGE: &str = "Simple CRUD API with Rust, SQLX, Postgres,and Axum";

// registration
// check
// activate
// users

pub async fn health_checker_handler() -> impl IntoResponse {
	let json_response = serde_json::json!({
		"status": "success",
		"message": MESSAGE
	});

	return Json(json_response);
}

// pub async fn index_handler(Path(params): Path<HashMap<String, String>>) -> impl IntoResponse {
// 	println!("params: {:?}", params);
// 	return "<p>Hello, World!</p>";
// }

pub async fn favicon_handler() -> impl IntoResponse {
	return StatusCode::NO_CONTENT;
}
