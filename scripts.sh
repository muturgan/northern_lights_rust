#!/bin/sh

build() {
	cargo build --release;
}

dev() {
	export $(cat .env | xargs) && cargo run
}

start_release() {
	export $(cat .env | xargs) && ./target/release/promo_codes
}

start() {
	build;
	start_release;
}

clippy() {
	cargo clippy --all --all-features --tests -- -D warnings;
	echo "clippy is ok";
}

fmt() {
	cargo fmt -- --check;
	echo "fmt is ok";
}

lint() {
	clippy;
	fmt;
}

"$@"
