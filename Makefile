
build:
	cargo build && cp ./target/debug/fnctl ./fnctl

build-reselase:
	cargo build --release && cp ./target/debug/fnctl ./fnctl
