# Makefile for a Rust project

# Extract package name from Cargo.toml
PACKAGE := $(shell grep 'name = ' Cargo.toml | head -n 1 | cut -d '"' -f 2)

# Set default binary name to be the same as package name
BINARY_NAME ?= $(PACKAGE)

# Default target is run
.PHONY: all
all: run

# Run target
.PHONY: run
run:
  # Modify this e.g. when using leptos or dx cli
	cargo run -p $(PACKAGE) --bin $(BINARY_NAME)

# Test target
.PHONY: test
test:
	cargo test

# Clean up
.PHONY: clean
clean:
	cargo clean
