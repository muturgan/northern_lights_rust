use super::super::Store;
use crate::repository::models::{InsertedPromo, RegisteredUser, UsersPromo};
use crate::system_models::AppError;
use ::std::sync::{Arc, Mutex};
use chrono::{DateTime, NaiveDate, Utc};

#[derive(Clone)]
struct MockUser {
	id: u32,
	firstname: String,
	birthdate: NaiveDate,
	phone: String,
	email: Option<String>,
	created_at: DateTime<Utc>,
	promocode: String,
	activated_at: Option<DateTime<Utc>>,
}

impl MockUser {
	fn to_user(&self) -> RegisteredUser {
		return RegisteredUser {
			id: self.id,
			firstname: self.firstname.clone(),
			birthdate: self.birthdate,
			phone: self.phone.clone(),
			email: self.email.clone(),
			created_at: self.created_at,
			promo: vec![UsersPromo {
				promocode: self.promocode.clone(),
				activated_at: self.activated_at,
			}],
		};
	}
}

#[derive(Clone)]
pub struct MockStore {
	store: Arc<Mutex<Vec<MockUser>>>,
}

impl MockStore {
	pub fn new() -> Self {
		Self {
			store: Arc::new(Mutex::new(Vec::new())),
		}
	}
}

impl Store for MockStore {
	async fn insert_user_and_grant_promo(
		&self,
		firstname: String,
		birthdate: NaiveDate,
		phone: String,
		promocode: String,
	) -> Result<InsertedPromo, AppError> {
		let current_store = self.store.lock();
		let mut current_store = match current_store {
			Err(err) => {
				return Err(AppError::SystemError(err.to_string()));
			}
			Ok(st) => st,
		};

		let existing_user = current_store.iter().find(|u| u.phone == phone);
		if existing_user.is_some() {
			return Err(AppError::ScenarioError(
				format!("Пользователь с номером телефона {phone} уже существует"),
				Some(phone),
			));
		}

		let new_user = MockUser {
			id: current_store.len() as u32 + 1,
			firstname,
			birthdate,
			phone,
			email: None,
			created_at: Utc::now(),
			promocode,
			activated_at: None,
		};

		let inserted_promo = new_user.promocode.clone();

		current_store.push(new_user);

		return Ok(InsertedPromo {
			promocode: inserted_promo,
		});
	}

	async fn read_users(&self) -> Result<Vec<RegisteredUser>, AppError> {
		let current_store = self.store.lock();
		let current_store = match current_store {
			Err(err) => {
				return Err(AppError::SystemError(err.to_string()));
			}
			Ok(st) => st,
		};
		return Ok(current_store.iter().map(|user| user.to_user()).collect());
	}

	async fn close(&self) {}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_to_user() {
		let mock_user = MockUser {
			id: 1,
			firstname: String::from("a"),
			birthdate: NaiveDate::default(),
			phone: String::from("7"),
			email: None,
			created_at: DateTime::default(),
			promocode: String::from("p"),
			activated_at: None,
		};

		let user = mock_user.to_user();

		assert_eq!(user.id, mock_user.id);
	}
}
