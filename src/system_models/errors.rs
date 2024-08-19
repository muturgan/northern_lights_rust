pub enum AppError {
	ScenarioError(String, Option<String>),
	SystemError(String),
}
