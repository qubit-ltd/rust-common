/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Error and Result Helpers
//!
//! Provides shared boxed error helpers and result aliases used by generic
//! callbacks, integration glue, and source error storage.
//!
//! This module intentionally contains only type-erased error helpers. Concrete
//! domain errors remain in their owning modules, such as
//! [`crate::lang::argument::ArgumentError`] and
//! [`crate::lang::converter::DataConversionError`].
//!
//! Use these helpers when the exact error type is deliberately unimportant or
//! cannot be expressed conveniently, for example callback return types, generic
//! defaults, and stored source errors. Prefer concrete error enums or structs
//! for public APIs where callers need structured recovery.
//!
//! # Examples
//!
//! ```rust
//! use qubit_common::lang::error::{BoxError, BoxResult, IntoBoxError};
//!
//! fn parse_port(text: &str) -> BoxResult<u16> {
//!     text.parse::<u16>()
//!         .map_err(|error| error.into_box_error())
//! }
//!
//! let port = parse_port("8080").expect("valid port should parse");
//! assert_eq!(port, 8080);
//!
//! let error: BoxError = parse_port("bad").expect_err("invalid port should fail");
//! assert!(error.to_string().contains("invalid digit"));
//! ```
//!
//! # Author
//!
//! Haixing Hu

mod box_error;
mod box_result;
mod dyn_error;
mod into_box_error;

pub use box_error::BoxError;
pub use box_result::BoxResult;
pub use dyn_error::DynError;
pub use into_box_error::IntoBoxError;
