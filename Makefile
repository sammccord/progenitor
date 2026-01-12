# Makefile for building and publishing Progenitor packages
#
# Usage:
#   make build            - Build all packages in dependency order
#   make publish-dry-run  - Test publish without uploading to crates.io
#   make publish          - Publish packages to crates.io with automatic delays

# Package names in dependency order
IMPL_PKG = progenitor-middleware-impl
MACRO_PKG = progenitor-middleware-macro
MIDDLEWARE_PKG = progenitor-middleware
CARGO_PKG = cargo-progenitor

# Registry update delay (in seconds)
PUBLISH_DELAY = 120

.PHONY: build publish publish-dry-run help

# Default target shows help
help:
	@echo "Progenitor Package Management"
	@echo ""
	@echo "Available targets:"
	@echo "  make build            - Build all packages in dependency order"
	@echo "  make publish-dry-run  - Test publish without uploading to crates.io"
	@echo "  make publish          - Publish packages to crates.io (includes delays)"
	@echo ""
	@echo "Publish order:"
	@echo "  1. $(IMPL_PKG)"
	@echo "  2. $(MACRO_PKG)"
	@echo "  3. $(MIDDLEWARE_PKG)"
	@echo "  4. $(CARGO_PKG)"

# Build all packages in dependency order
build:
	@echo "=========================================="
	@echo "Building packages in dependency order..."
	@echo "=========================================="
	@echo ""
	@echo "[1/4] Building $(IMPL_PKG)..."
	cargo build -p $(IMPL_PKG)
	@echo ""
	@echo "[2/4] Building $(MACRO_PKG)..."
	cargo build -p $(MACRO_PKG)
	@echo ""
	@echo "[3/4] Building $(MIDDLEWARE_PKG)..."
	cargo build -p $(MIDDLEWARE_PKG)
	@echo ""
	@echo "[4/4] Building $(CARGO_PKG)..."
	cargo build -p $(CARGO_PKG)
	@echo ""
	@echo "=========================================="
	@echo "✓ All packages built successfully!"
	@echo "=========================================="

# Dry-run publish to verify everything is ready
publish-dry-run:
	@echo "=========================================="
	@echo "Dry-run publishing packages..."
	@echo "=========================================="
	@echo ""
	@echo "[1/4] Dry-run publishing $(IMPL_PKG)..."
	cargo publish --dry-run -p $(IMPL_PKG)
	@echo ""
	@echo "[2/4] Dry-run publishing $(MACRO_PKG)..."
	cargo publish --dry-run -p $(MACRO_PKG)
	@echo ""
	@echo "[3/4] Dry-run publishing $(MIDDLEWARE_PKG)..."
	cargo publish --dry-run -p $(MIDDLEWARE_PKG)
	@echo ""
	@echo "[4/4] Dry-run publishing $(CARGO_PKG)..."
	cargo publish --dry-run -p $(CARGO_PKG)
	@echo ""
	@echo "=========================================="
	@echo "✓ Dry-run completed successfully!"
	@echo "=========================================="

# Publish packages to crates.io with delays between each
publish:
	@echo "=========================================="
	@echo "Publishing packages to crates.io..."
	@echo "=========================================="
	@echo ""
	@echo "[1/4] Publishing $(IMPL_PKG)..."
	cargo publish -p $(IMPL_PKG)
	@echo ""
	@echo "⏱  Waiting $(PUBLISH_DELAY) seconds for crates.io to update..."
	sleep $(PUBLISH_DELAY)
	@echo ""
	@echo "[2/4] Publishing $(MACRO_PKG)..."
	cargo publish -p $(MACRO_PKG)
	@echo ""
	@echo "⏱  Waiting $(PUBLISH_DELAY) seconds for crates.io to update..."
	sleep $(PUBLISH_DELAY)
	@echo ""
	@echo "[3/4] Publishing $(MIDDLEWARE_PKG)..."
	cargo publish -p $(MIDDLEWARE_PKG)
	@echo ""
	@echo "⏱  Waiting $(PUBLISH_DELAY) seconds for crates.io to update..."
	sleep $(PUBLISH_DELAY)
	@echo ""
	@echo "[4/4] Publishing $(CARGO_PKG)..."
	cargo publish -p $(CARGO_PKG)
	@echo ""
	@echo "=========================================="
	@echo "✓ All packages published successfully!"
	@echo "=========================================="
