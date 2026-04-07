/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # DataType Unit Tests
//!
//! Tests for data type enum functionality.
//!
//! # Author
//!
//! Haixing Hu

use qubit_common::DataType;

/// Test DataType::as_str method for all data types
#[test]
fn test_data_type_as_str_all_types() {
    // Boolean type
    assert_eq!(DataType::Bool.as_str(), "bool");

    // Character type
    assert_eq!(DataType::Char.as_str(), "char");

    // Signed integer types
    assert_eq!(DataType::Int8.as_str(), "int8");
    assert_eq!(DataType::Int16.as_str(), "int16");
    assert_eq!(DataType::Int32.as_str(), "int32");
    assert_eq!(DataType::Int64.as_str(), "int64");
    assert_eq!(DataType::Int128.as_str(), "int128");

    // Unsigned integer types
    assert_eq!(DataType::UInt8.as_str(), "uint8");
    assert_eq!(DataType::UInt16.as_str(), "uint16");
    assert_eq!(DataType::UInt32.as_str(), "uint32");
    assert_eq!(DataType::UInt64.as_str(), "uint64");
    assert_eq!(DataType::UInt128.as_str(), "uint128");

    // Floating point types
    assert_eq!(DataType::Float32.as_str(), "float32");
    assert_eq!(DataType::Float64.as_str(), "float64");

    // String type
    assert_eq!(DataType::String.as_str(), "string");

    // Date and time types
    assert_eq!(DataType::Date.as_str(), "date");
    assert_eq!(DataType::Time.as_str(), "time");
    assert_eq!(DataType::DateTime.as_str(), "datetime");
    assert_eq!(DataType::Instant.as_str(), "instant");

    // Big number types
    assert_eq!(DataType::BigInteger.as_str(), "biginteger");
    assert_eq!(DataType::BigDecimal.as_str(), "bigdecimal");
}

/// Test DataType Display trait implementation
#[test]
fn test_data_type_display_all_types() {
    // Boolean type
    assert_eq!(DataType::Bool.to_string(), "bool");

    // Character type
    assert_eq!(DataType::Char.to_string(), "char");

    // Signed integer types
    assert_eq!(DataType::Int8.to_string(), "int8");
    assert_eq!(DataType::Int16.to_string(), "int16");
    assert_eq!(DataType::Int32.to_string(), "int32");
    assert_eq!(DataType::Int64.to_string(), "int64");
    assert_eq!(DataType::Int128.to_string(), "int128");

    // Unsigned integer types
    assert_eq!(DataType::UInt8.to_string(), "uint8");
    assert_eq!(DataType::UInt16.to_string(), "uint16");
    assert_eq!(DataType::UInt32.to_string(), "uint32");
    assert_eq!(DataType::UInt64.to_string(), "uint64");
    assert_eq!(DataType::UInt128.to_string(), "uint128");

    // Floating point types
    assert_eq!(DataType::Float32.to_string(), "float32");
    assert_eq!(DataType::Float64.to_string(), "float64");

    // String type
    assert_eq!(DataType::String.to_string(), "string");

    // Date and time types
    assert_eq!(DataType::Date.to_string(), "date");
    assert_eq!(DataType::Time.to_string(), "time");
    assert_eq!(DataType::DateTime.to_string(), "datetime");
    assert_eq!(DataType::Instant.to_string(), "instant");

    // Big number types
    assert_eq!(DataType::BigInteger.to_string(), "biginteger");
    assert_eq!(DataType::BigDecimal.to_string(), "bigdecimal");
}

/// Test DataType Clone and Copy traits
#[test]
fn test_data_type_clone_copy() {
    let dt1 = DataType::Int32;
    let dt2 = dt1; // Copy
    let dt3 = dt1; // Copy (DataType implements Copy, so direct assignment is sufficient)

    assert_eq!(dt1, dt2);
    assert_eq!(dt1, dt3);
    assert_eq!(dt2, dt3);
}

/// Test DataType PartialEq and Eq traits
#[test]
fn test_data_type_equality() {
    assert_eq!(DataType::Bool, DataType::Bool);
    assert_ne!(DataType::Bool, DataType::Int32);

    assert_eq!(DataType::Int32, DataType::Int32);
    assert_ne!(DataType::Int32, DataType::Int64);

    assert_eq!(DataType::String, DataType::String);
    assert_ne!(DataType::String, DataType::Date);
}

/// Test DataType Debug trait
#[test]
fn test_data_type_debug() {
    // Ensure all types can be properly Debug output
    let types = vec![
        DataType::Bool,
        DataType::Char,
        DataType::Int8,
        DataType::Int16,
        DataType::Int32,
        DataType::Int64,
        DataType::Int128,
        DataType::UInt8,
        DataType::UInt16,
        DataType::UInt32,
        DataType::UInt64,
        DataType::UInt128,
        DataType::Float32,
        DataType::Float64,
        DataType::String,
        DataType::Date,
        DataType::Time,
        DataType::DateTime,
        DataType::Instant,
        DataType::BigInteger,
        DataType::BigDecimal,
    ];

    for dt in types {
        let debug_str = format!("{:?}", dt);
        assert!(!debug_str.is_empty(), "Debug output should not be empty");
    }
}
