use promo_codes::*;

#[test]
fn read_bips() {
	let bips = config::get_bips();
	assert!(bips.len() > 0);
}
