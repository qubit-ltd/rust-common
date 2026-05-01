/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Boxed Error Result
//!
//! Defines the result alias for operations that intentionally return boxed
//! dynamic errors.
//!
//! `BoxResult<T>` is a concise spelling of `Result<T, BoxError>` for examples,
//! callbacks, and integration helpers that combine unrelated error types.
//!
//! # Author
//!
//! Haixing Hu

use super::BoxError;

/// Result alias using [`BoxError`] as the error type.
///
/// This is primarily useful for examples, callbacks, integration glue, and
/// internal helpers that combine several unrelated error types.
///
/// # Design Rationale
///
/// A repeated `Result<T, BoxError>` signature is noisy in higher-order APIs.
/// This alias keeps those signatures focused on the successful value while
/// still making the erased error type explicit through the `Box` prefix.
///
/// `BoxResult<T>` intentionally has only one type parameter. If an API should
/// preserve a caller-provided error type, write `Result<T, E>` directly instead
/// of hiding `E` behind a boxed trait object.
///
/// # When to Use
///
/// Use `BoxResult<T>` when every failure path can be treated uniformly as a
/// type-erased source error, especially in:
///
/// - examples and small executable entry points;
/// - callback return types;
/// - internal orchestration that combines file, parse, and configuration
///   errors without structured recovery.
///
/// Avoid it for domain APIs that should expose a concrete error enum.
///
/// # Examples
///
/// Combining unrelated error types with `?`:
///
/// ```rust
/// use std::fs;
///
/// use qubit_common::lang::error::BoxResult;
///
/// fn read_port(path: &str) -> BoxResult<u16> {
///     let content = fs::read_to_string(path)?;
///     let port = content.trim().parse::<u16>()?;
///     Ok(port)
/// }
/// ```
///
/// Using `BoxResult` in a callback-oriented helper:
///
/// ```rust
/// use qubit_common::lang::error::BoxResult;
///
/// fn run_task<F>(task: F) -> BoxResult<usize>
/// where
///     F: FnOnce() -> BoxResult<usize>,
/// {
///     task()
/// }
///
/// let value = run_task(|| Ok(42)).expect("task should succeed");
/// assert_eq!(value, 42);
/// ```
pub type BoxResult<T> = Result<T, BoxError>;
