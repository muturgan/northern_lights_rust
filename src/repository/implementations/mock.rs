use super::super::Store;
use crate::repository::models::{InsertedPromo, RegisteredUser, UsersPromo};
use crate::system_models::AppError;
use ::std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use chrono::{DateTime, NaiveDate, Utc};

impl<'a, T: ?Sized + 'a> From<PoisonError<MutexGuard<'a, T>>> for AppError {
	fn from(err: PoisonError<MutexGuard<'a, T>>) -> Self {
		return AppError::SystemError(err.to_string());
	}
}

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
		firstname: &str,
		birthdate: NaiveDate,
		phone: &str,
		promocode: &str,
	) -> Result<InsertedPromo, AppError> {
		let mut current_store = self.store.lock()?;

		let existing_user = current_store.iter().find(|u| u.phone == phone);
		if existing_user.is_some() {
			return AppError::user_already_exists(phone.to_string()).into();
		}

		let new_user = MockUser {
			id: current_store.len() as u32 + 1,
			firstname: firstname.to_string(),
			birthdate,
			phone: phone.to_string(),
			email: None,
			created_at: Utc::now(),
			promocode: promocode.to_string(),
			activated_at: None,
		};

		let inserted_promo = new_user.promocode.clone();

		current_store.push(new_user);

		return Ok(InsertedPromo {
			promocode: inserted_promo,
		});
	}

	async fn check_promo(&self, user_phone: &str, promocode: &str) -> Result<(), AppError> {
		let current_store = self.store.lock()?;

		let existing_user = current_store.iter().find(|u| u.phone == user_phone);
		if existing_user.is_none() {
			return AppError::promo_not_exists().into();
		}

		let existing_user = existing_user.unwrap();

		if existing_user.promocode != promocode {
			return AppError::promo_not_exists().into();
		}

		return match existing_user.activated_at {
			Some(_) => AppError::promo_already_activated().into(),
			None => Ok(()),
		};
	}

	async fn activate_promo(&self, _user_phone: &str, _promocode: &str) -> Result<(), AppError> {
		todo!()
	}

	async fn read_users(&self) -> Result<Vec<RegisteredUser>, AppError> {
		let current_store = self.store.lock()?;
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
