// Copyright 2024 Oxide Computer Company

//! Progenitor is a Rust crate for generating opinionated clients from API
//! descriptions specified in the OpenAPI 3.0.x format. It makes use of Rust
//! futures for async API calls and `Streams` for paginated interfaces.
//!
//! It generates a type called `Client` with methods that correspond to the
//! operations specified in the OpenAPI document.
//!
//! For details see the [repo
//! README](https://github.com/oxidecomputer/progenitor/blob/main/README.md)

#![deny(missing_docs)]

#[cfg(feature = "macro")]
pub use progenitor_middleware_client;
pub use progenitor_middleware_impl::CrateVers;
pub use progenitor_middleware_impl::Error;
pub use progenitor_middleware_impl::GenerationSettings;
pub use progenitor_middleware_impl::Generator;
pub use progenitor_middleware_impl::InterfaceStyle;
pub use progenitor_middleware_impl::TagStyle;
pub use progenitor_middleware_impl::TypeImpl;
pub use progenitor_middleware_impl::TypePatch;
#[cfg(feature = "macro")]
pub use progenitor_middleware_macro::generate_api;
