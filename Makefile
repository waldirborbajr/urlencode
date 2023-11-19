build:
	cargo watch -q -c -w src/ -x "build -q"

run:
	cargo watch -q -c -w src/ -x "run -q"

clean:
	cargo clean

cache:
	cargo-cache --remove-dir all

install:
	cargo build --release
	cargo install --path .
