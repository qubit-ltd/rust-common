/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Dynamic Error Object
//!
//! Defines the shared dynamic error object bounds used by boxed errors.
//!
//! `DynError` names the trait object behind [`super::BoxError`]. Keeping the
//! bounds in one alias avoids repeating `dyn Error + Send + Sync + 'static`
//! across callback signatures and source-error storage fields.
//!
//! Most APIs should use [`super::BoxError`] for owned errors. Use `DynError`
//! directly when the error is borrowed behind another pointer or reference.
//!
//! # Author
//!
//! Haixing Hu

use std::error::Error;

/// Dynamic error trait object used by shared boxed error helpers.
///
/// The object is `Send`, `Sync`, and `'static` so it can be moved across
/// thread and async task boundaries.
///
/// # Design Rationale
///
/// Error trait objects are dynamically sized, so they must be accessed behind a
/// pointer such as `&`, `Box`, or `Arc`. This alias captures the project-wide
/// bounds for type-erased errors:
///
/// - `Send` allows the error object to cross thread and task boundaries.
/// - `Sync` allows shared references to the error object to be used safely.
/// - `'static` prevents borrowed context from being hidden inside stored errors.
///
/// # When to Use
///
/// Use `DynError` when an API needs to accept or inspect a borrowed dynamic
/// error object without taking ownership. For owned error values, prefer
/// [`super::BoxError`].
///
/// # Examples
///
/// ```rust
/// use std::io;
///
/// use qubit_common::lang::error::DynError;
///
/// fn render_error(error: &DynError) -> String {
///     format!("operation failed: {error}")
/// }
///
/// let error = io::Error::other("disk is full");
/// assert_eq!(render_error(&error), "operation failed: disk is full");
/// ```
pub type DynError = dyn Error + Send + Sync + 'static;
