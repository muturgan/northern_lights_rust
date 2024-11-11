use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
#[cfg(feature = "postgres")]
use sqlx::{FromRow, types::Json as SqlxJson};

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "postgres", derive(FromRow))]
pub struct User {
	pub id: i32,
	pub firstname: String,
	pub birthdate: NaiveDate,
	pub phone: String,
	pub email: Option<String>,
	pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "postgres", derive(FromRow))]
pub struct Promo {
	pub promocode: String,
	pub holder_id: u32,
	pub activated_at: Option<DateTime<Utc>>,
	pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "postgres", derive(FromRow))]
pub struct CheckResult {
	pub promocode: String,
	pub phone: String,
	pub activated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "postgres", derive(FromRow))]
pub struct ActivationResult {
	pub activated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "postgres", derive(FromRow))]
pub struct InsertedPromo {
	pub promocode: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "postgres", derive(FromRow))]
pub struct UsersPromo {
	pub promocode: String,
	pub activated_at: Option<DateTime<Utc>>,
}

#[cfg(feature = "postgres")]
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
	#[serde(rename = "ID")]
	pub id: u32,
	pub firstname: String,
	pub birthdate: NaiveDate,
	pub phone: String,
	pub email: Option<String>,
	pub created_at: DateTime<Utc>,
	pub promo: Vec<UsersPromo>,
}

#[cfg(feature = "postgres")]
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
