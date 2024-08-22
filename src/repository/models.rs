use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{types::Json as SqlxJson, FromRow};

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct User {
	#[sqlx(try_from = "i32")]
	pub id: u32,
	pub firstname: String,
	pub birthdate: NaiveDate,
	pub phone: String,
	pub email: Option<String>,
	pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Promo {
	pub promocode: String,
	pub holder_id: u32,
	pub activated_at: Option<DateTime<Utc>>,
	pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct InsertedPromo {
	pub promocode: String,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct UsersPromo {
	pub promocode: String,
	pub activated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct RegisteredUserRow {
	#[sqlx(try_from = "i32")]
	pub id: u32,
	pub firstname: String,
	pub birthdate: NaiveDate,
	pub phone: String,
	pub email: Option<String>,
	pub created_at: DateTime<Utc>,
	pub promo: SqlxJson<Vec<UsersPromo>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisteredUser {
	pub id: u32,
	pub firstname: String,
	pub birthdate: NaiveDate,
	pub phone: String,
	pub email: Option<String>,
	pub created_at: DateTime<Utc>,
	pub promo: Vec<UsersPromo>,
}

impl From<RegisteredUserRow> for RegisteredUser {
	fn from(user: RegisteredUserRow) -> Self {
		let SqlxJson(promo) = user.promo;

		return RegisteredUser {
			id: user.id,
			firstname: user.firstname,
			birthdate: user.birthdate,
			phone: user.phone,
			email: user.email,
			created_at: user.created_at,
			promo,
		};
	}
}
