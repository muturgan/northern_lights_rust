use ::std::env::{current_dir, var as readEnvVar};
use ::std::fs::File;
use ::std::io::{prelude::*, BufReader};
use ::std::net::SocketAddr;
use ::std::path::Path;

pub fn get_http_host_to_serve() -> SocketAddr {
	let app_host = readEnvVar("APP_HOST").expect("APP_HOST environment variable is not defined");

	format!("{app_host}:80")
		.parse::<SocketAddr>()
		.expect("APP_HOST is not a correct IP address");

	let app_port = readEnvVar("APP_PORT")
		.expect("APP_PORT environment variable is not defined")
		.parse::<u16>()
		.expect("APP_PORT is not a correct u16");

	let host_to_parse = format!("{app_host}:{app_port}");

	return host_to_parse
		.parse()
		.unwrap_or_else(|_| panic!("Unable to parse socket address for {app_host}:{app_port}"));
}

pub fn get_db_config() -> String {
	let db_host = readEnvVar("DB_HOST").expect("DB_HOST environment variable is not defined");

	let db_port = readEnvVar("DB_PORT")
		.expect("DB_PORT environment variable is not defined")
		.parse::<u16>()
		.expect("DB_PORT is not a correct u16");

	let db_name = readEnvVar("DB_NAME").expect("DB_NAME environment variable is not defined");
	let db_user = readEnvVar("DB_USER").expect("DB_USER environment variable is not defined");
	let db_pass = readEnvVar("DB_PASS").expect("DB_PASS environment variable is not defined");

	return format!("postgresql://{db_user}:{db_pass}@{db_host}:{db_port}/{db_name}?schema=public");
}

pub fn get_db_max_pool_size() -> u32 {
	let default_pool_size = String::from("10");

	return u32::from(
		readEnvVar("DB_MAX_POOL_SIZE")
			.unwrap_or(default_pool_size)
			.parse::<u8>()
			.expect("DB_MAX_POOL_SIZE is not a correct u8"),
	);
}

pub fn get_bips() -> Vec<String> {
	let cwd = current_dir().unwrap();
	let cwd = cwd.to_str().unwrap();
	let bips_path = Path::new(cwd).join("bip39_russian.txt");

	let mut bips = Vec::new();

	let file = File::open(bips_path).unwrap();
	let reader = BufReader::new(file);
	for line in reader.lines() {
		bips.push(line.unwrap());
	}

	return bips;
}

pub fn is_test() -> bool {
	return match readEnvVar("ENV") {
		Err(_) => false,
		Ok(val) => val == "test",
	};
}
