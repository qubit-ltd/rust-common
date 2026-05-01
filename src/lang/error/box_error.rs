/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Boxed Error
//!
//! Defines the boxed dynamic error alias used in generic and callback-oriented
//! contexts.
//!
//! `BoxError` is the owned form of [`super::DynError`]. It erases the concrete
//! error type while preserving the standard [`std::error::Error`] interface and
//! the thread-safety bounds required by async and threaded code.
//!
//! # Author
//!
//! Haixing Hu

use super::DynError;

/// Boxed dynamic error used when a concrete error type is intentionally erased.
///
/// Prefer concrete error types for public APIs that callers need to inspect or
/// recover from. Use this alias for callback signatures, generic defaults, and
/// source error storage where type erasure is intentional.
///
/// # Design Rationale
///
/// `dyn Error` is a dynamically sized type and cannot be returned or stored by
/// value. Boxing gives it a stable owned representation:
///
/// - `Box` owns the concrete error value behind dynamic dispatch.
/// - [`super::DynError`] centralizes the `Error + Send + Sync + 'static` bounds.
/// - The alias keeps callback and generic signatures readable.
///
/// A `BoxError` should not replace structured domain errors. Public APIs that
/// need caller recovery should expose concrete error types, usually an enum or
/// struct. `BoxError` is best for glue code and places where the caller only
/// needs to log, propagate, or store the original source.
///
/// # When to Use
///
/// Use `BoxError` for:
///
/// - callbacks that may fail with several unrelated error types;
/// - default generic error parameters where callers may still choose a concrete
///   error type;
/// - storing an optional source error inside a concrete outer error;
/// - application or example code where precise recovery is not required.
///
/// Avoid `BoxError` when callers need to match on variants, retry by category,
/// or present structured diagnostics.
///
/// # Examples
///
/// Returning a boxed source error with the `?` operator:
///
/// ```rust
/// use std::fs;
///
/// use qubit_common::lang::error::BoxError;
///
/// fn load_config(path: &str) -> Result<String, BoxError> {
///     let content = fs::read_to_string(path)?;
///     Ok(content)
/// }
/// ```
///
/// Storing a type-erased source error inside a concrete wrapper:
///
/// ```rust
/// use std::io;
///
/// use qubit_common::lang::error::BoxError;
///
/// struct TaskFailure {
///     source: BoxError,
/// }
///
/// let failure = TaskFailure {
///     source: Box::new(io::Error::other("worker stopped")),
/// };
///
/// assert_eq!(failure.source.to_string(), "worker stopped");
/// ```
pub type BoxError = Box<DynError>;
