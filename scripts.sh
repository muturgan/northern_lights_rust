#!/bin/sh

build() {
	cargo build --release;
}

dev() {
	export $(cat .env | xargs) && cargo run
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
