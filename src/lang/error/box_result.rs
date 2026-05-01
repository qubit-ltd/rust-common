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

use super::BoxError;

/// Result alias using [`BoxError`] as the error type.
///
/// This is primarily useful for examples, callbacks, integration glue, and
/// internal helpers that combine several unrelated error types.
pub type BoxResult<T> = Result<T, BoxError>;
