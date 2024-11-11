mod implementations;
pub mod models;

use chrono::NaiveDate;
#[cfg(not(feature = "postgres"))]
use implementations::MockStore;
#[cfg(feature = "postgres")]
use implementations::PostgresStore;
use models::{InsertedPromo, RegisteredUser};

use crate::system_models::AppError;

trait Store {
	async fn insert_user_and_grant_promo(
		&self,
		first_name: &str,
		birth_date: NaiveDate,
		phone: &str,
		promocode: &str,
	) -> Result<InsertedPromo, AppError>;

	async fn check_promo(&self, user_phone: &str, promocode: &str) -> Result<(), AppError>;

	async fn activate_promo(&self, user_phone: &str, promocode: &str) -> Result<(), AppError>;

	async fn read_users(&self) -> Result<Vec<RegisteredUser>, AppError>;

	async fn close(&self);
}

#[derive(Clone)]
pub struct Repository {
	#[cfg(not(feature = "postgres"))]
	store: MockStore,

	#[cfg(feature = "postgres")]
	store: PostgresStore,
}

impl Repository {
	pub async fn new() -> Self {
		return Self {
			#[cfg(not(feature = "postgres"))]
			store: MockStore::new(),

			#[cfg(feature = "postgres")]
			store: PostgresStore::new().await,
		};
	}

	pub async fn insert_user_and_grant_promo(
		&self,
		first_name: &str,
		birth_date: NaiveDate,
		phone: &str,
		promocode: &str,
	) -> Result<InsertedPromo, AppError> {
		return self
			.store
			.insert_user_and_grant_promo(first_name, birth_date, phone, promocode)
			.await;
	}

	pub async fn check_promo(&self, user_phone: &str, promocode: &str) -> Result<(), AppError> {
		return self.store.check_promo(user_phone, promocode).await;
	}

	pub async fn activate_promo(&self, user_phone: &str, promocode: &str) -> Result<(), AppError> {
		return self.store.activate_promo(user_phone, promocode).await;
	}

	pub async fn read_users(&self) -> Result<Vec<RegisteredUser>, AppError> {
		return self.store.read_users().await;
	}

	pub async fn close(&self) {
		return self.store.close().await;
	}
}
