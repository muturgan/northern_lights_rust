#![allow(clippy::upper_case_acronyms)]
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum EScenarioStatus {
	SCENARIO_SUCCESS,
	UNAUTHORIZED,
	SCENARIO_FAIL,
	SYSTEM_ERROR,
}

impl<'de> Deserialize<'de> for EScenarioStatus {
	fn deserialize<D>(deserializer: D) -> Result<EScenarioStatus, D::Error>
	where
		D: Deserializer<'de>,
	{
		let numval = u8::deserialize(deserializer);
		return match numval {
			Err(e) => Err(e),
			Ok(num) => {
				return match num {
					0 => Ok(EScenarioStatus::SCENARIO_SUCCESS),
					1 => Ok(EScenarioStatus::UNAUTHORIZED),
					2 => Ok(EScenarioStatus::SCENARIO_FAIL),
					3 => Ok(EScenarioStatus::SYSTEM_ERROR),
					_ => Err(D::Error::custom(String::from("incorrect scenario status"))),
				};
			}
		};
	}
}

impl Serialize for EScenarioStatus {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let numval: u8 = match self {
			EScenarioStatus::SCENARIO_SUCCESS => 0,
			EScenarioStatus::UNAUTHORIZED => 1,
			EScenarioStatus::SCENARIO_FAIL => 2,
			EScenarioStatus::SYSTEM_ERROR => 3,
		};
		return serializer.serialize_u8(numval);
	}
}
