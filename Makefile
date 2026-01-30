# Makefile for Dungeon Crawler World
# Provides build targets for CI/CD workflows

.PHONY: all test build clean check

# Default target - build everything
all: check test

# Run tests without requiring graphics libraries
# Set SKIP_GRAPHICS to bypass Vulkan/X11 linking in build.rs
test:
	SKIP_GRAPHICS=1 cargo test

# Build the main application
build:
	cargo build --release

# Run clippy and other checks
# Set SKIP_GRAPHICS to bypass Vulkan/X11 linking in build.rs
check:
	SKIP_GRAPHICS=1 cargo check --lib

# Clean build artifacts
clean:
	cargo clean

# Help target
help:
	@echo "Available targets:"
	@echo "  all    - Run checks and tests (default)"
	@echo "  test   - Run unit tests without graphics dependencies"
	@echo "  build  - Build release binary"
	@echo "  check  - Run cargo check"
	@echo "  clean  - Clean build artifacts"
	@echo "  help   - Show this help message"
