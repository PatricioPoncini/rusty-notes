build:
	@cargo build -p axum-with-sqlx
.PHONY: build

test:
	@bash scripts/run_tests.sh
.PHONY: test

format:
	@cargo fmt
.PHONY: format