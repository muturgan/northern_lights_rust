use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

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
pub struct InsertedUser {
	pub id: i32,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct InsertedPromo {
	pub promocode: String,
}
