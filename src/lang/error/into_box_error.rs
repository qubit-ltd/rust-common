/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Boxed Error Conversion
//!
//! Provides an extension trait for explicitly boxing concrete error values.
//!
//! Many `BoxResult<T>` functions can rely on the `?` operator to convert
//! concrete errors into [`BoxError`]. `IntoBoxError` covers the cases where the
//! conversion must be written as an expression, such as `map_err` closures or
//! manually constructed `Err` values.
//!
//! # Author
//!
//! Haixing Hu

use std::error::Error;

use super::BoxError;

/// Extension trait for converting concrete errors into [`BoxError`].
///
/// In many `BoxResult<T>` functions, the `?` operator can perform this
/// conversion automatically. This trait is useful when a call site needs an
/// explicit conversion, for example inside `map_err`.
///
/// # Design Rationale
///
/// The standard conversion from a concrete error to `Box<dyn Error + ...>` is
/// available through `From`, but writing it explicitly often requires a verbose
/// cast. This trait gives the conversion a project-specific name and keeps call
/// sites readable:
///
/// ```rust
/// # use qubit_common::lang::error::IntoBoxError;
/// # fn demo() -> qubit_common::lang::error::BoxResult<u16> {
/// let port = "8080"
///     .parse::<u16>()
///     .map_err(|error| error.into_box_error())?;
/// # Ok(port)
/// # }
/// ```
///
/// # When to Use
///
/// Use `IntoBoxError` when a concrete error must be converted explicitly. If a
/// function already returns [`BoxResult`](super::BoxResult), prefer plain `?`
/// when inference is clear.
///
/// # Examples
///
/// Explicit conversion in `map_err`:
///
/// ```rust
/// use qubit_common::lang::error::{BoxResult, IntoBoxError};
///
/// fn parse_port(text: &str) -> BoxResult<u16> {
///     text.parse::<u16>()
///         .map_err(|error| error.into_box_error())
/// }
///
/// assert_eq!(parse_port("8080").expect("valid port"), 8080);
/// assert!(parse_port("not-a-port").is_err());
/// ```
///
/// Manual `Err` construction:
///
/// ```rust
/// use std::io;
///
/// use qubit_common::lang::error::{BoxResult, IntoBoxError};
///
/// fn require_enabled(enabled: bool) -> BoxResult<()> {
///     if enabled {
///         Ok(())
///     } else {
///         Err(io::Error::other("feature disabled").into_box_error())
///     }
/// }
///
/// assert!(require_enabled(true).is_ok());
/// assert_eq!(
///     require_enabled(false)
///         .expect_err("disabled feature should fail")
///         .to_string(),
///     "feature disabled"
/// );
/// ```
pub trait IntoBoxError: Error + Send + Sync + 'static + Sized {
    /// Converts this error into a boxed dynamic error.
    ///
    /// # Returns
    ///
    /// A [`BoxError`] preserving this error as the boxed source value.
    ///
    /// # Allocation
    ///
    /// This method allocates one `Box` and stores `self` as the boxed concrete
    /// source error. It does not perform I/O, block, or otherwise inspect the
    /// error value.
    #[inline]
    fn into_box_error(self) -> BoxError {
        Box::new(self)
    }
}

impl<E> IntoBoxError for E where E: Error + Send + Sync + 'static {}
