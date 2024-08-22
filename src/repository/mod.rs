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
		first_name: String,
		birth_date: NaiveDate,
		phone: String,
		promocode: String,
	) -> Result<InsertedPromo, AppError>;

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
		first_name: String,
		birth_date: NaiveDate,
		phone: String,
		promocode: String,
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
