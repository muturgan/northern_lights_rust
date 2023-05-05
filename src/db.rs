use ::std::sync::Arc;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::config;

pub struct AppState {
	pub db: Pool<Postgres>,
}

pub async fn create_db_connection() -> Arc<AppState> {
	let database_url = config::get_db_config();
	let pool = PgPoolOptions::new()
		.max_connections(config::get_db_max_pool_size())
		.connect(&database_url)
		.await
		.expect(":( Failed to connect to the database");
	println!(":) Connection to the database is successful");

	sqlx::migrate!("./migrations")
		.run(&pool)
		.await
		.expect(":( Migrations failed");
	println!(":) Migrations finished");

	let app_state = Arc::new(AppState { db: pool });
	return app_state;
}
