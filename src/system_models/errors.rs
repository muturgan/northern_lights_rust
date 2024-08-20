use ::std::error::Error;
use ::std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub enum AppError {
	ScenarioError(String, Option<String>),
	SystemError(String),
}

impl Display for AppError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}", self)
	}
}

impl Error for AppError {}
