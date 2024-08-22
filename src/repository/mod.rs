mod implementations;
pub mod models;

use crate::config;
use crate::system_models::AppError;
use chrono::NaiveDate;
use implementations::{MockStore, PostgresStore};
use models::{InsertedPromo, RegisteredUser};

#[derive(Clone)]
enum StoreKind {
	Mock(MockStore),
	Postgres(PostgresStore),
}

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
	store: StoreKind,
}

impl Repository {
	pub async fn new() -> Self {
		if config::is_test() {
			return Self {
				store: StoreKind::Mock(MockStore::new()),
			};
		}

		return Self {
			store: StoreKind::Postgres(PostgresStore::new().await),
		};
	}

	pub async fn insert_user_and_grant_promo(
		&self,
		first_name: &str,
		birth_date: NaiveDate,
		phone: &str,
		promocode: &str,
	) -> Result<InsertedPromo, AppError> {
		match &self.store {
			StoreKind::Mock(store) => {
				return store
					.insert_user_and_grant_promo(first_name, birth_date, phone, promocode)
					.await
			}
			StoreKind::Postgres(store) => {
				return store
					.insert_user_and_grant_promo(first_name, birth_date, phone, promocode)
					.await
			}
		};
	}

	pub async fn check_promo(&self, user_phone: &str, promocode: &str) -> Result<(), AppError> {
		return match &self.store {
			StoreKind::Mock(store) => store.check_promo(user_phone, promocode).await,
			StoreKind::Postgres(store) => store.check_promo(user_phone, promocode).await,
		};
	}

	pub async fn activate_promo(&self, user_phone: &str, promocode: &str) -> Result<(), AppError> {
		return match &self.store {
			StoreKind::Mock(store) => store.activate_promo(user_phone, promocode).await,
			StoreKind::Postgres(store) => store.activate_promo(user_phone, promocode).await,
		};
	}

	pub async fn read_users(&self) -> Result<Vec<RegisteredUser>, AppError> {
		return match &self.store {
			StoreKind::Mock(store) => store.read_users().await,
			StoreKind::Postgres(store) => store.read_users().await,
		};
	}

	pub async fn close(&self) {
		match &self.store {
			StoreKind::Mock(store) => store.close().await,
			StoreKind::Postgres(store) => store.close().await,
		};
	}
}
