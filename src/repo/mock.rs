use super::{Repo, RepoError};
use crate::models::{InsertedPromo, User};
use chrono::{DateTime, NaiveDate, Utc};
use std::cell::RefCell;

const STORE: RefCell<Vec<MockUser>> = RefCell::new(Vec::new());

struct MockUser {
	id: u32,
	firstname: String,
	birthdate: NaiveDate,
	phone: String,
	email: Option<String>,
	created_at: DateTime<Utc>,
	promocode: String,
}

impl MockUser {
	fn to_user(&self) -> User {
		return User {
			id: self.id,
			firstname: self.firstname.clone(),
			birthdate: self.birthdate.clone(),
			phone: self.phone.clone(),
			email: self.email.clone(),
			created_at: self.created_at.clone(),
		};
	}
}

pub struct MockStore {}

impl MockStore {
	pub fn new() -> Self {
		Self {}
	}
}

impl Repo for MockStore {
	async fn insert_user_and_grant_promo(
		&self,
		firstname: String,
		birthdate: NaiveDate,
		phone: String,
		promocode: String,
	) -> Result<InsertedPromo, RepoError> {
		let new_user = MockUser {
			id: STORE.borrow().len() as u32,
			firstname,
			birthdate,
			phone: phone,
			email: None,
			created_at: DateTime::default(),
			promocode,
		};

		let inserted_promo = new_user.promocode.clone();

		STORE.borrow_mut().push(new_user);

		return Ok(InsertedPromo {
			promocode: inserted_promo,
		});
	}

	async fn read_users(&self) -> Result<Vec<User>, sqlx::Error> {
		return Ok(STORE.borrow().iter().map(|user| user.to_user()).collect());
	}
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
		};

		let user = mock_user.to_user();

		assert_eq!(user.id, mock_user.id);
	}
}
