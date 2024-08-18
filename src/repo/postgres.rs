use super::{Repo, RepoError};
use crate::models::{InsertedPromo, User};
use chrono::NaiveDate;
use sqlx::PgPool;

struct PostgresStore<'a> {
	pool: &'a PgPool,
}

impl<'a> PostgresStore<'a> {
	fn new(pool: &'a PgPool) -> Self {
		Self { pool }
	}
}

impl<'a> Repo for PostgresStore<'a> {
	async fn insert_user_and_grant_promo(
		&self,
		first_name: String,
		birth_date: NaiveDate,
		phone: String,
		promocode: String,
	) -> Result<InsertedPromo, RepoError> {
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
		.fetch_one(self.pool)
		.await;

		return match query_result {
			Err(err) => {
				let err_str = err.to_string();
				if err_str.contains("duplicate key") {
					Err(RepoError::AlreadyExists(phone))
				} else {
					Err(RepoError::Fail(err_str))
				}
			}
			Ok(p) => Ok(p),
		};
	}

	async fn read_users(&self) -> Result<Vec<User>, sqlx::Error> {
		return sqlx::query_as::<_, User>("SELECT * FROM users")
			.fetch_all(self.pool)
			.await;
	}
}
