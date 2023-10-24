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
	@echo "Linting all projects with cargo"
	@rustup component add clippy 2> /dev/null
	./lint.sh

test:
	cargo test --quiet

run:
	cargo run

release:
	cargo build --release

all: format lint test run
