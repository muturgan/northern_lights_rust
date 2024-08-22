mod pool;

use super::super::Store;
use crate::repository::models::{CheckResult, InsertedPromo, RegisteredUser, RegisteredUserRow};
use crate::system_models::AppError;
use chrono::{NaiveDate, Utc};
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
				Err(if err_str.contains("duplicate key") {
					AppError::user_already_exists(phone)
				} else {
					AppError::SystemError(err_str)
				})
			}
			Ok(p) => Ok(p),
		};
	}

	async fn check_promo(&self, user_phone: String, promocode: String) -> Result<(), AppError> {
		let mut promos = sqlx::query_as::<_, CheckResult>(
			"SELECT promocode, phone, activated_at FROM promo P
			INNER JOIN users U ON P.holder_id = U.ID
			WHERE promocode = $1 and phone = $2;",
		)
		.bind(promocode)
		.bind(user_phone)
		.fetch_all(&self.pool)
		.await?;

		let promo = promos.pop();

		return match promo {
			None => Err(AppError::promo_not_exists()),
			Some(p) => match p.activated_at {
				Some(_) => Err(AppError::promo_already_activated()),
				None => Ok(()),
			},
		};
	}

	async fn activate_promo(&self, user_phone: String, promocode: String) -> Result<(), AppError> {
		self.check_promo(user_phone, promocode.clone()).await?;

		let query_result = sqlx::query("UPDATE promo SET activated_at = $1 WHERE promocode = $2;")
			.bind(Utc::now())
			.bind(promocode)
			.execute(&self.pool)
			.await?;

		if query_result.rows_affected() != 1 {
			return Err(AppError::SystemError(String::from(
				"Не удалось выполнить UPDATE для активации промокода",
			)));
		}

		return Ok(());
	}

	async fn read_users(&self) -> Result<Vec<RegisteredUser>, AppError> {
		let users_list = sqlx::query_as::<_, RegisteredUserRow>(
			"SELECT u.*,
			json_agg(json_build_object('promocode', p.promocode, 'activated_at', p.activated_at)) as promo
			FROM users u
			LEFT JOIN promo p ON u.id = p.holder_id
			GROUP BY u.id
			ORDER BY u.created_at ASC;",
		)
		.fetch_all(&self.pool)
		.await?;

		return Ok(users_list.into_iter().map(RegisteredUser::from).collect());
	}

	async fn close(&self) {
		self.pool.close().await;
	}
}
