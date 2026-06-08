.PHONY: test coverage lint format audit migrate-add migrate-run create-db prepare

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

migrate-add:
	@test -n "$(name)" || (echo "Usage: make migrate-add name=create_initial_tables"; exit 1)
	sqlx migrate add -r $(name)

migrate-run:
	sqlx migrate run

# must set DATABASE_URL in .env file before running this command
create-db:
	sqlx database create

prepare:
	cargo sqlx prepare --workspace