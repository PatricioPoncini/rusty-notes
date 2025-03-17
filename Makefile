build:
	@cargo build -p rusty_notes
.PHONY: build

test:
	@bash scripts/run_tests.sh
.PHONY: test

format:
	@cargo fmt
.PHONY: format