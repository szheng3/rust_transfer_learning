PARAMETER = default

rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

format-check:
	cargo check


run:
	cargo run -- text -i "$(PARAMETER)"


benchx86:
	cargo bench

releasex86:
	cargo build --release

web:
	cd frontend && yarn install && yarn build && cp -R ./dist ../



all: format lint test run
