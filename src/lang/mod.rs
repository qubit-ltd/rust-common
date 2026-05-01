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
pub mod data_converter;
pub mod data_type;
pub mod data_type_of;

pub use box_error::{
    BoxError,
    BoxResult,
};
pub use data_converter::{
    DataConversionError,
    DataConversionResult,
    DataConvertTo,
    DataConverter,
};
pub use data_type::DataType;
pub use data_type_of::DataTypeOf;

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
