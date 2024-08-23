use ::std::sync::Arc;
use axum::extract::State;

use promo_codes::*;
use system_models::EScenarioStatus;

#[tokio::test]
async fn test_workflow() {
	let repo = repository::Repository::new().await;
	let repo = Arc::new(repo);

	let state_repo = State(repo.clone());

	let users = handler::users(state_repo.clone()).await.unwrap();
	assert_eq!(users.status, EScenarioStatus::SCENARIO_SUCCESS);
	println!("{}", users.payload.unwrap());

	repo.close().await;
}
