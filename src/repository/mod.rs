mod mock;
mod postgr;

use chrono::NaiveDate;

use crate::config;
use crate::models::{InsertedPromo, User};
use crate::system_models::AppError;

#[derive(Clone)]
enum StoreKind {
	Mock(mock::MockStore),
	Postgres(postgr::PostgresStore),
}

trait Store {
	async fn insert_user_and_grant_promo(
		&self,
		first_name: String,
		birth_date: NaiveDate,
		phone: String,
		promocode: String,
	) -> Result<InsertedPromo, AppError>;

	async fn read_users(&self) -> Result<Vec<User>, AppError>;

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
				store: StoreKind::Mock(mock::MockStore::new()),
			};
		}

		return Self {
			store: StoreKind::Postgres(postgr::PostgresStore::new().await),
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

	pub async fn read_users(&self) -> Result<Vec<User>, AppError> {
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
