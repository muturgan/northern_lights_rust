mod pool;

use chrono::NaiveDate;
use sqlx::{Error as EqlxError, PgPool};

use super::super::Store;
use crate::{
	repository::models::{ActivationResult, CheckResult, InsertedPromo, RegisteredUser},
	system_models::{AppError, CoreResult},
};

impl From<EqlxError> for AppError {
	fn from(err: EqlxError) -> Self {
		return AppError::system_error(err);
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
		first_name: &str,
		birth_date: NaiveDate,
		phone: &str,
		promocode: &str,
	) -> CoreResult<InsertedPromo> {
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
		.bind(phone)
		.bind(promocode)
		.fetch_one(&self.pool)
		.await;

		return match query_result {
			Err(err) => {
				let err_str = err.to_string();
				if err_str.contains("duplicate key") {
					AppError::user_already_exists(phone.to_string()).into()
				} else {
					AppError::SystemError(err_str).into()
				}
			}
			Ok(p) => Ok(p),
		};
	}

	async fn check_promo(&self, user_phone: &str, promocode: &str) -> CoreResult {
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
			None => AppError::promo_not_exists().into(),
			Some(p) => match p.activated_at {
				Some(_) => AppError::promo_already_activated().into(),
				None => Ok(()),
			},
		};
	}

	async fn activate_promo(&self, user_phone: &str, promocode: &str) -> CoreResult {
		let mut activation_result = sqlx::query_as::<_, ActivationResult>(
			"WITH before_update AS (
			SELECT promocode, holder_id, activated_at FROM promo P
			INNER JOIN users U ON P.holder_id = U.ID
			WHERE promocode = $1 and phone = $2
		),
		updated_promo AS (
			UPDATE promo
			SET activated_at = NOW()
			WHERE holder_id in (SELECT holder_id FROM before_update)
			AND activated_at IS NULL
		)
		SELECT before_update.activated_at
		FROM before_update",
		)
		.bind(promocode)
		.bind(user_phone)
		.fetch_all(&self.pool)
		.await?;

		let activation_result = activation_result.pop();

		return match activation_result {
			None => AppError::promo_not_exists().into(),
			Some(result) => match result.activated_at {
				Some(_) => AppError::promo_already_activated().into(),
				None => Ok(()),
			},
		};
	}

	async fn read_users(&self) -> CoreResult<Vec<RegisteredUser>> {
		let users_list = sqlx::query_as::<_, RegisteredUser>(
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
