#!/bin/sh

build() {
	cargo build --release;
}

dev() {
	export $(cat .env | grep -v '^#' | xargs) && cargo run
}

start_release() {
	export $(cat .env | grep -v '^#' | xargs) && ./target/release/promo_codes
}

start() {
	build;
	start_release;
}

test() {
	cargo test
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
