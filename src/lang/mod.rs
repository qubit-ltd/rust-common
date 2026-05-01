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
pub mod converter;
pub mod datatype;

pub use box_error::{
    BoxError,
    BoxResult,
};
pub use converter::{
    DataConversionError,
    DataConversionResult,
    DataConvertTo,
    DataConverter,
};
pub use datatype::{
    DataType,
    DataTypeOf,
    DataTypeParseError,
};

// Re-export commonly used types
pub use argument::{
    ArgumentError,
    ArgumentResult,
    CollectionArgument,
    NumericArgument,
    OptionArgument,
    StringArgument,
    check_argument,
    check_state,
};
