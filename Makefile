# Default target
.DEFAULT_GOAL := help

BIN_NAME := rmlm
BUILD_DIR := target/release
RELEASE_DIR := release

# List of supported platform targets
PLATFORMS := linux-x64 linux-arm64 win-x64 mac-arm64 mac-x64 mac-universal

# Mark all targets as phony (not actual files)
.PHONY: help build run fmt fmt-check lint test build-all release $(foreach platform, $(PLATFORM_MAP), $(platform))

help:
	@echo "make <command>:"
	@echo "  make build          - Build the project"
	@echo "  make run            - Run the project"
	@echo "  make fmt            - Format code"
	@echo "  make fmt-check      - Check format code"
	@echo "  make lint           - Run clippy linter"
	@echo "  make test           - Run tests"
	@echo "  make build-all      - Build for all platforms"
	@echo "  make release        - Copy all generated binaries to the release directory"
	@echo "  make <platform>     - Build for a specific platform (e.g., make linux-x64)"

build:
	cargo build --release

run:
	cargo run -- $(ARGS)

fmt:
	cargo fmt

fmt-check:
	cargo fmt -- --check

lint:
	cargo clippy -- -D warnings

test:
	cargo test

build-all:
	@echo "Building for all platforms..."
	@for platform in $(PLATFORMS); do \
		$(MAKE) $$platform || exit 1; \
	done


release: build-all
	@echo "Creating release directory..."
	mkdir -p $(RELEASE_DIR)

	@echo "Copying binaries to release directory..."
	@cp target/x86_64-unknown-linux-musl/release/$(BIN_NAME)        $(RELEASE_DIR)/$(BIN_NAME)-linux-x64
	@cp target/aarch64-unknown-linux-musl/release/$(BIN_NAME)       $(RELEASE_DIR)/$(BIN_NAME)-linux-arm64
	@cp target/x86_64-pc-windows-gnu/release/$(BIN_NAME).exe        $(RELEASE_DIR)/$(BIN_NAME)-win-x64.exe
	@cp target/aarch64-apple-darwin/release/$(BIN_NAME)             $(RELEASE_DIR)/$(BIN_NAME)-mac-arm64
	@cp target/x86_64-apple-darwin/release/$(BIN_NAME)              $(RELEASE_DIR)/$(BIN_NAME)-mac-x64
	@cp target/mac-universal/release/$(BIN_NAME)                    $(RELEASE_DIR)/$(BIN_NAME)-mac-universal

# Generic rule to build a platform
build-platform:
	@echo "Building for platform: $(platform_target)"
	cargo build --release --target $(platform_target)

# Define explicit rules for each platform and assign the platform_target
linux-x64:
	$(MAKE) build-platform platform_target=x86_64-unknown-linux-musl

linux-arm64:
	$(MAKE) build-platform platform_target=aarch64-unknown-linux-musl

win-x64:
	$(MAKE) build-platform platform_target=x86_64-pc-windows-gnu

mac-arm64:
	$(MAKE) build-platform platform_target=aarch64-apple-darwin

mac-x64:
	$(MAKE) build-platform platform_target=x86_64-apple-darwin

mac-universal:
	@echo "Building for mac-arm64..."
	$(MAKE) mac-arm64
	@echo "Building for mac-x64..."
	$(MAKE) mac-x64
	@echo "Creating universal binary using lipo..."
	@mkdir -p target/mac-universal/release
	lipo -create target/aarch64-apple-darwin/release/$(BIN_NAME) target/x86_64-apple-darwin/release/$(BIN_NAME) -output target/mac-universal/release/$(BIN_NAME)
	@echo "Universal binary created at target/mac-universal/release/$(BIN_NAME)"

