use super::{Repo, RepoError};
use crate::models::{InsertedPromo, User};
use chrono::{DateTime, NaiveDate, Utc};
use std::cell::RefCell;

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

struct MockRepo {
	store: RefCell<Vec<MockUser>>,
}

impl MockRepo {
	fn new() -> Self {
		Self {
			store: RefCell::new(Vec::new()),
		}
	}
}

impl Repo for MockRepo {
	async fn insert_user_and_grant_promo(
		&self,
		firstname: String,
		birthdate: NaiveDate,
		phone: String,
		promocode: String,
	) -> Result<InsertedPromo, RepoError> {
		let mut xs = vec![1i32, 2, 3];
		xs.push(2);

		let new_user = MockUser {
			id: self.store.borrow().len() as u32,
			firstname,
			birthdate,
			phone: phone,
			email: None,
			created_at: DateTime::default(),
			promocode,
		};

		let inserted_promo = new_user.promocode.clone();

		self.store.borrow_mut().push(new_user);

		return Ok(InsertedPromo {
			promocode: inserted_promo,
		});
	}

	async fn read_users(&self) -> Result<Vec<User>, sqlx::Error> {
		return Ok(self
			.store
			.borrow()
			.iter()
			.map(|user| user.to_user())
			.collect());
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
