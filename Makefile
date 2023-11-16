build:
	cargo watch -q -c -w src/ -x "build -q"

run:
	cargo watch -q -c -w src/ -x "run -q"

install:
	cargo build --release
	cargo install --path .
