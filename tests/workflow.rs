use ::std::sync::Arc;
use axum::http::{StatusCode, header};
use axum_test::TestServer;
use promo_codes::{
	repository::{Repository, models::RegisteredUser},
	router,
	system_models::{AppResponse, EScenarioStatus},
};
use serde_json::json;

#[tokio::test]
async fn test_workflow() {
	let repo = Repository::new().await;
	let app = router::create_router(Arc::new(repo));
	let server = TestServer::new(app).unwrap();

	// Пустой репозиторий
	let res = server.get("/api/users").await;
	assert_eq!(res.status_code(), StatusCode::OK);
	let body = res.json::<AppResponse>();
	assert_eq!(body.status, EScenarioStatus::SCENARIO_SUCCESS);
	let users = serde_json::from_value::<Vec<RegisteredUser>>(body.payload.unwrap()).unwrap();
	assert_eq!(users.len(), 0);

	// Регистрация
	let res = server
		.post("/api/registration")
		.json(&json!({
			"phone": "+79505901234",
			"firstName": "Andrey",
			"birthDate": "1987-01-21",
		}))
		.await;
	assert_eq!(res.status_code(), StatusCode::OK);
	let body = res.json::<AppResponse>();
	assert_eq!(body.status, EScenarioStatus::SCENARIO_SUCCESS);
	let mut iter = body.result.split(": ");
	iter.next();
	let promo = iter.next().unwrap().to_lowercase();

	// Непустой репозиторий
	let res = server.get("/api/users").await;
	assert_eq!(res.status_code(), StatusCode::OK);
	let body = res.json::<AppResponse>();
	assert_eq!(body.status, EScenarioStatus::SCENARIO_SUCCESS);
	let users = serde_json::from_value::<Vec<RegisteredUser>>(body.payload.unwrap()).unwrap();
	assert_eq!(users.len(), 1);

	// Ошибка авторизации
	let res = server.post("/api/check").await;
	assert_eq!(res.status_code(), StatusCode::OK);
	let body = res.json::<AppResponse>();
	assert_eq!(body.status, EScenarioStatus::UNAUTHORIZED);

	// Проверка промокода
	let pass = promo_codes::config::get_admin_pass();
	let res = server
		.post("/api/check")
		.add_header(header::AUTHORIZATION, pass)
		.json(&json!({
			"phone": "+79505901234",
			"promocode": &promo,
		}))
		.await;
	assert_eq!(res.status_code(), StatusCode::OK);
	let body = res.json::<AppResponse>();
	assert_eq!(body.status, EScenarioStatus::SCENARIO_SUCCESS);
	let auth_cookie = res.cookies().delta().next().unwrap().to_owned();

	// Активация промокода
	let res = server
		.post("/api/activate")
		.add_cookie(auth_cookie.clone())
		.json(&json!({
			"phone": "+79505901234",
			"promocode": &promo,
		}))
		.await;
	assert_eq!(res.status_code(), StatusCode::OK);
	let body = res.json::<AppResponse>();
	assert_eq!(body.status, EScenarioStatus::SCENARIO_SUCCESS);

	// Повторная активация промокода
	let res = server
		.post("/api/activate")
		.add_cookie(auth_cookie.clone())
		.json(&json!({
			"phone": "+79505901234",
			"promocode": &promo,
		}))
		.await;
	assert_eq!(res.status_code(), StatusCode::OK);
	let body = res.json::<AppResponse>();
	assert_eq!(body.status, EScenarioStatus::SCENARIO_FAIL);
}
