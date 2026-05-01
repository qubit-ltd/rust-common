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
