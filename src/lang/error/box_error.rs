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

use super::DynError;

/// Boxed dynamic error used when a concrete error type is intentionally erased.
///
/// Prefer concrete error types for public APIs that callers need to inspect or
/// recover from. Use this alias for callback signatures, generic defaults, and
/// source error storage where type erasure is intentional.
pub type BoxError = Box<DynError>;
