//! Rust platform registry: provides programmatic access to information about valid Rust platforms
//!
//! This crate provides an interface to the platform data available at Rust Forge:
//!
//! <https://forge.rust-lang.org/platform-support.html>

#![crate_name = "platforms"]
#![crate_type = "lib"]
#![deny(warnings, missing_docs, trivial_casts, trivial_numeric_casts)]
#![deny(unused_import_braces, unused_qualifications)]
#![forbid(unsafe_code)]
#![no_std]
#![doc(html_root_url = "https://docs.rs/platforms/0.0.0")]

#[cfg(feature = "std")]
extern crate std;

/// Rust platform types
pub mod platform;

/// Rust target types
pub mod target;

#[cfg(feature = "std")]
pub use platform::PlatformReq;
pub use platform::{Platform, ALL_PLATFORMS};
pub use target::{TARGET_ARCH, TARGET_ENV, TARGET_OS};

/// Find a Rust platform by its "target triple", e.g. `i686-apple-darwin`
pub fn find<S: AsRef<str>>(target_triple: S) -> Option<&'static Platform> {
    ALL_PLATFORMS
        .iter()
        .find(|platform| platform.target_triple == target_triple.as_ref())
}
