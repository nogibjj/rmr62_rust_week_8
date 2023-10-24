rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format:
	cd test_v_python
	cargo fmt --quiet

lint:
	@pwd
	cd test_v_python
	cargo clippy --quiet

test:
	cd test_v_python
	cargo test --quiet

run:
	cd test_v_python
	cargo run

release:
	cd test_v_python
	cargo build --release

all: format lint test run
