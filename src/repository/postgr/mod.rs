mod pool;

use super::Store;
use crate::models::{InsertedPromo, User};
use crate::system_models::AppError;
use chrono::NaiveDate;
use sqlx::{Error as EqlxError, PgPool};

impl From<EqlxError> for AppError {
	fn from(err: EqlxError) -> Self {
		return AppError::SystemError(err.to_string());
	}
}

#[derive(Clone)]
pub struct PostgresStore {
	pool: PgPool,
}

impl PostgresStore {
	pub async fn new() -> Self {
		let pool = pool::create_db_connection().await;
		Self { pool }
	}
}

impl Store for PostgresStore {
	async fn insert_user_and_grant_promo(
		&self,
		first_name: String,
		birth_date: NaiveDate,
		phone: String,
		promocode: String,
	) -> Result<InsertedPromo, AppError> {
		let query_result = sqlx::query_as::<_, InsertedPromo>(
			"WITH inserted_user AS (
				INSERT INTO users (firstname,birthdate,phone) VALUES ($1, $2, $3) RETURNING id
			)
			INSERT INTO promo (promocode,holder_id)
			SELECT $4 as promocode, id FROM inserted_user
			RETURNING promocode;",
		)
		.bind(first_name)
		.bind(birth_date)
		.bind(&phone)
		.bind(promocode)
		.fetch_one(&self.pool)
		.await;

		return match query_result {
			Err(err) => {
				let err_str = err.to_string();
				if err_str.contains("duplicate key") {
					Err(AppError::ScenarioError(
						format!("Пользователь с номером телефона {phone} уже существует"),
						Some(phone),
					))
				} else {
					Err(AppError::SystemError(err_str))
				}
			}
			Ok(p) => Ok(p),
		};
	}

	async fn read_users(&self) -> Result<Vec<User>, AppError> {
		return Ok(sqlx::query_as::<_, User>("SELECT * FROM users")
			.fetch_all(&self.pool)
			.await?);
	}

	async fn close(&self) {
		self.pool.close().await;
	}
}
