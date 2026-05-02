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
pub mod converter;
pub mod datatype;
pub mod error;

pub use converter::{
    DataConversionError,
    DataConversionResult,
    DataConvertTo,
    DataConverter,
    DataConverters,
    DataListConversionError,
    DataListConversionResult,
};
pub use datatype::{
    DataType,
    DataTypeOf,
    DataTypeParseError,
};
pub use error::{
    BoxError,
    BoxResult,
    DynError,
    IntoBoxError,
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
