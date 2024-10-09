# Makefile

.PHONY: format lint build

# Target to format the code
format:
	cargo fmt

# Target to run the linter
lint:
	cargo clippy

# Target to build the project
build:
	cargo build

# Target to run all tasks: format, lint, and build
all: format lint build
