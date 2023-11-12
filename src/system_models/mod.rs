pub mod api_response;
pub mod scenario_status;

pub struct AppError {
	pub message: String,
	pub payload: Option<String>,
}
