.PHONY: test coverage lint format audit

test:
	cargo test

coverage:
	cargo tarpaulin --ignore-tests

lint:
	cargo clippy

format:
	cargo fmt

audit:
	cargo audit