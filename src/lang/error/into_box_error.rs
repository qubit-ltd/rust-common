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

use std::error::Error;

use super::BoxError;

/// Extension trait for converting concrete errors into [`BoxError`].
///
/// In many `BoxResult<T>` functions, the `?` operator can perform this
/// conversion automatically. This trait is useful when a call site needs an
/// explicit conversion, for example inside `map_err`.
pub trait IntoBoxError: Error + Send + Sync + 'static + Sized {
    /// Converts this error into a boxed dynamic error.
    ///
    /// # Returns
    ///
    /// A [`BoxError`] preserving this error as the boxed source value.
    #[inline]
    fn into_box_error(self) -> BoxError {
        Box::new(self)
    }
}

impl<E> IntoBoxError for E where E: Error + Send + Sync + 'static {}
