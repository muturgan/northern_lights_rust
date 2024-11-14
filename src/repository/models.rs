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

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "postgres", derive(FromRow))]
pub struct RegisteredUser {
	#[serde(rename = "ID")]
	#[cfg_attr(feature = "postgres", sqlx(try_from = "i32"))]
	pub id: u32,
	pub firstname: String,
	pub birthdate: NaiveDate,
	pub phone: String,
	pub email: Option<String>,
	pub created_at: DateTime<Utc>,

	#[cfg(not(feature = "postgres"))]
	pub promo: Vec<UsersPromo>,
	#[cfg(feature = "postgres")]
	pub promo: SqlxJson<Vec<UsersPromo>>,
}
