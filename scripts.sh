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
	export $(cat .env | grep -v '^#' | xargs) && cargo test
}

bin() {
	export $(cat .env | grep -v '^#' | xargs) && cargo run --bin workflow
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
