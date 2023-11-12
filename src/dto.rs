use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RegistrationDto {
	pub phone: String,
	pub firstName: String,
	pub birthDate: String,
}
