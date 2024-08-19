use ::std::sync::Arc;
use axum::Extension;

use promo_codes::*;
use system_models::EScenarioStatus;

#[tokio::test]
async fn test_workflow() {
	let repo = repository::Repository::new().await;
	let repo = Arc::new(repo);

	let ext_repo = Extension(repo.clone());

	let users = handler::users(ext_repo.clone()).await;
	assert_eq!(users.status, EScenarioStatus::SCENARIO_SUCCESS);
	println!("{}", users.payload.unwrap());

	repo.close().await;
}
