# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Progenitor is a Rust code generator that creates type-safe, opinionated API clients from OpenAPI 3.0.x specifications. It generates async clients using `reqwest-middleware` with support for streaming, pagination, and customizable hooks.

## Workspace Structure

This is a Cargo workspace with 8 packages organized into distinct roles:

- **progenitor-middleware** (0.12.3) - Main facade crate that re-exports impl and macro
- **progenitor-middleware-impl** (0.12.3) - Core code generation logic (~5,600 LOC)
- **progenitor-middleware-client** (0.12.0) - Runtime support types for generated clients
- **progenitor-middleware-macro** (0.12.3) - Procedural macro for `generate_api!`
- **cargo-progenitor** (0.12.3) - Binary cargo plugin for static crate generation
- **example-macro**, **example-build**, **example-wasm** - Usage examples

### Key Files in progenitor-middleware-impl

- `lib.rs` - Main Generator struct and GenerationSettings
- `method.rs` (2,325 LOC) - HTTP method generation (largest file)
- `cli.rs` - CLI generation using clap
- `httpmock.rs` - Mock generation for testing
- `to_schema.rs` - OpenAPI schema processing

## Common Commands

### Build and Test
```bash
# Build all packages
cargo build

# Run all tests
cargo test

# Run only library tests
cargo test --lib

# Run specific integration test
cargo test --test build_keeper

# Build examples
cargo build -p example-macro
cargo build -p example-build
```

### Using the cargo-progenitor Plugin
```bash
# Install the plugin locally
cargo install --path cargo-progenitor

# Generate a client crate from OpenAPI spec
cargo progenitor -i sample_openapi/keeper.json -o output_dir -n crate_name -v 0.1.0

# With options
cargo progenitor -i spec.json -o output \
  --interface Builder \
  --tags Separate \
  --license MIT \
  --include-client true
```

### Testing with Sample APIs
Test coverage uses OpenAPI specs in `sample_openapi/`:
- keeper.json, buildomat.json, nexus.json, propolis-server.json (Oxide APIs)
- api.github.com.json, api.coinbase.com.json (external APIs)

## Architecture

### Generation Pipeline
```
OpenAPI Spec (JSON/YAML)
  ↓ openapiv3::parse
OpenAPI AST
  ↓ type extraction
Typify (JSON Schema → Rust Types)
  ↓ method generation (method.rs)
proc-macro2::TokenStream
  ↓ formatting
Formatted Rust Code
```

### Three Usage Modes

1. **Macro** - Inline generation via `generate_api!("spec.json")`
2. **build.rs** - Generation in build script using `Generator::generate_tokens()`
3. **Static Crate** - Pre-generated crate via `cargo progenitor` CLI

### Interface Styles

- **Positional** (default): `client.create_item(org, project, body).await?`
- **Builder**: `client.create_item().org(org).project(project).body(body).send().await?`

### Response Types

Generated methods return:
```rust
Result<ResponseValue<T>, Error<E>>
```

Where:
- `ResponseValue<T>` wraps response with `.status()`, `.headers()`, and derefs to `T`
- `Error<E>` has 7 variants including `ErrorResponse(ResponseValue<E>)` for API errors

## Testing Infrastructure

### Test Organization
- **progenitor-middleware/tests/** - Integration tests against real APIs
- **progenitor-middleware-impl/tests/** - Snapshot tests using `expectorate`
  - **test_output.rs** - Generates code for all styles and compares to golden files
  - **output/src/** - Golden files for snapshot comparison
- **cargo-progenitor/tests/** - CLI command tests

### Running Snapshot Tests
The `test_output.rs` uses `expectorate` to compare generated code against golden files in `tests/output/src/`. If generation changes:
1. Review the diff carefully
2. Update golden files if changes are intentional: set `EXPECTORATE=overwrite` env var
3. Commit updated golden files with your changes

## Code Generation Details

### Generator State
The `Generator` struct maintains:
- `TypeSpace` from typify for schema → type mapping
- `settings: GenerationSettings` for configuration
- Internal state about operations, tags, and schemas

### GenerationSettings Builder
Configure generation with:
```rust
Generator::new().with_settings(
    GenerationSettings::default()
        .with_interface(InterfaceStyle::Builder)
        .with_tag(TagStyle::Separate)
        .with_derive("schemars::JsonSchema")
        .with_pre_hook("my_pre_hook")
        .with_post_hook("my_post_hook")
)
```

### Customization Points
- **Hooks**: pre/post request handlers (sync or async variants)
- **Type patches**: Override generated types via `TypePatch`
- **Extra derives**: Add custom derive macros to all generated types
- **Crate mapping**: Redirect crate imports (e.g., use custom `chrono` fork)

## Dependencies

Key external dependencies (see workspace.dependencies in root Cargo.toml):
- **openapiv3** (2.2.0) - OpenAPI spec parsing
- **typify** (0.4.2) - Type generation from JSON schemas
- **reqwest** (0.12.4) + **reqwest-middleware** (0.4.2) - HTTP client with middleware
- **proc-macro2**, **quote**, **syn** - Rust code generation
- **prettyplease** (0.2.21+) - Rust code formatting
- **clap** (4.5.43) - CLI generation
- **tokio** (1.45.1) - Async runtime

## Release Process

Uses `cargo-release` with config in `release.toml`:
- All packages version together (shared version)
- CHANGELOG.adoc is auto-updated
- Workspace dependencies are automatically bumped

```bash
# Dry run
cargo release

# Execute release
cargo release --execute
```

## Current Branch Context

Branch `sammccord/implement-reqwest-middleware` is integrating `reqwest-middleware` support to replace raw `reqwest` usage, enabling extensible HTTP middleware for generated clients.

## Documentation

- **README.md** - Primary user documentation with examples for all three usage modes
- **docs/progenitor-client.md** - Details on ResponseValue and Error types
- **docs/positional-generation.md** - Positional interface examples
- **docs/builder-generation.md** - Builder interface examples
- **docs/implementing-client.md** - Custom client implementation guide
