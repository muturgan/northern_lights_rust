build() {
	cargo build;
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
