// Copyright 2022 Oxide Computer Company

//! Support for generated clients.

#![deny(missing_docs)]

mod progenitor_middleware_client;

pub use crate::progenitor_middleware_client::*;

// For stand-alone crates, rather than adding a dependency on
// progenitor-middleware-client, we simply dump the code right in. This means we don't
// need to determine the provenance of progenitor (crates.io, github, etc.)
// when generating the stand-alone crate.
#[doc(hidden)]
pub fn code() -> &'static str {
    include_str!("progenitor_middleware_client.rs")
}
