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

use std::error::Error;

/// Dynamic error trait object used by shared boxed error helpers.
///
/// The object is `Send`, `Sync`, and `'static` so it can be moved across
/// thread and async task boundaries.
pub type DynError = dyn Error + Send + Sync + 'static;
