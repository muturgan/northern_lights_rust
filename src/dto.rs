use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct RegistrationDto {
	pub phone: String,
	pub firstName: String,
	#[serde(rename = "birthDate")]
	pub birth_date: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PromoDto {
	pub phone: String,
	pub promocode: String,
}
