/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Qubit Common - Core Utility Library
//!
//! Provides language-level fundamental tools and data type support, including:
//! - Data type definitions and validation
//! - Argument validation and error handling
//! - Core utility functions
//!
//! # Author
//!
//! Haixing Hu

pub mod lang;
#[path = "serde/mod.rs"]
mod serde_impl;
pub mod serde {
    pub use super::serde_impl::*;
}
pub mod util;

// Re-export main types from lang module
pub use lang::{
    argument::{
        ArgumentError,
        ArgumentResult,
        CollectionArgument,
        NumericArgument,
        OptionArgument,
        // String functions
        StringArgument,
        // Core functions
        check_argument,
        // Condition functions
        check_argument_fmt,
        check_argument_with_message,
        check_bounds,
        check_element_index,
        check_position_index,
        check_position_indexes,
        check_state,
        check_state_with_message,
        // Collection functions
        require_element_non_null,
        // Numeric functions
        require_equal,
        require_not_equal,
        // Option functions
        require_null_or,
    },
    converter::{
        DataConversionError,
        DataConversionResult,
        DataConvertTo,
        DataConverter,
        DataConverters,
        DataListConversionError,
        DataListConversionResult,
    },
    datatype::{
        DataType,
        DataTypeOf,
        DataTypeParseError,
    },
    error::{
        BoxError,
        BoxResult,
        DynError,
        IntoBoxError,
    },
};

// Re-export utility types
pub use util::{
    Pair,
    Triple,
};
