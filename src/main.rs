use promo_codes;

#[tokio::main]
async fn main() {
	promo_codes::start().await;
}
