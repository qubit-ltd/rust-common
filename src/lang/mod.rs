/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Language Tools
//!
//! Provides common language-level tools and utility functions.
//!
//! # Author
//!
//! Haixing Hu

pub mod argument;
pub mod box_error;
pub mod data_type;

pub use box_error::{
    BoxError,
    BoxResult,
};
pub use data_type::{
    DataType,
    DataTypeOf,
};

// Re-export commonly used types
pub use argument::{
    check_argument,
    check_state,
    ArgumentError,
    ArgumentResult,
    CollectionArgument,
    NumericArgument,
    OptionArgument,
    StringArgument,
};
