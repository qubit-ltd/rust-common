/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # DataConverters Unit Tests
//!
//! Tests for reusable batch data conversion behavior.
//!
//! # Author
//!
//! Haixing Hu

use std::error::Error;

use num_bigint::BigInt;
use qubit_common::lang::converter::{
    DataConversionError,
    DataConverters,
};

/// Test batch conversion from a borrowed vector without moving the source.
#[test]
fn test_data_converters_from_borrowed_vec_converts_all_values() {
    let values = vec!["8080".to_string(), "9090".to_string()];

    let ports: Vec<u16> = DataConverters::from(&values)
        .to_vec()
        .expect("borrowed string vector should convert to u16 vector");

    assert_eq!(ports, vec![8080, 9090]);
    assert_eq!(values, vec!["8080".to_string(), "9090".to_string()]);
}

/// Test batch conversion from a borrowed slice.
#[test]
fn test_data_converters_from_slice_converts_all_values() {
    let values = [1i32, 2, 3];

    let converted: Vec<i64> = DataConverters::from(values.as_slice())
        .to_vec()
        .expect("i32 slice should convert to i64 vector");

    assert_eq!(converted, vec![1i64, 2, 3]);
}

/// Test batch conversion from an owned vector.
#[test]
fn test_data_converters_from_owned_vec_converts_all_values() {
    let values = vec![1i32, 2, 3];

    let converted: Vec<String> = DataConverters::from(values)
        .to_vec()
        .expect("owned i32 vector should convert to string vector");

    assert_eq!(converted, vec!["1", "2", "3"]);
}

/// Test batch conversion from an iterator.
#[test]
fn test_data_converters_from_iterator_converts_all_values() {
    let values = ["1", "0", "true", "FALSE"];

    let converted: Vec<bool> = DataConverters::from_iterator(values.iter().copied())
        .to_vec()
        .expect("string iterator should convert to bool vector");

    assert_eq!(converted, vec![true, false, true, false]);
}

/// Test first-value conversion from a borrowed vector.
#[test]
fn test_data_converters_to_first_converts_first_value() {
    let values = vec![BigInt::from(42), BigInt::from(100)];

    let first: i32 = DataConverters::from(&values)
        .to_first()
        .expect("first BigInt should convert to i32");

    assert_eq!(first, 42);
}

/// Test empty batch conversion behavior.
#[test]
fn test_data_converters_empty_sources() {
    let values: Vec<String> = Vec::new();

    let converted: Vec<u16> = DataConverters::from(&values)
        .to_vec()
        .expect("empty batch should convert to empty vector");
    assert!(converted.is_empty());

    assert!(matches!(
        DataConverters::from(&values).to_first::<u16>(),
        Err(DataConversionError::NoValue)
    ));
}

/// Test that batch conversion errors preserve the failing element index.
#[test]
fn test_data_converters_error_contains_failing_index() {
    let values = vec!["1".to_string(), "bad".to_string(), "3".to_string()];

    let error = DataConverters::from(&values)
        .to_vec::<u16>()
        .expect_err("invalid second element should fail conversion");

    assert_eq!(error.index, 1);
    assert!(matches!(
        error.source,
        DataConversionError::ConversionError(_)
    ));
    assert!(
        error.to_string().contains("index 1"),
        "display should include failing index: {error}"
    );
    assert!(
        Error::source(&error).is_some(),
        "error source should expose the underlying conversion error"
    );
}

/// Test length helpers for exact-size sources.
#[test]
fn test_data_converters_len_and_is_empty() {
    let values = vec![1u8, 2, 3];
    let converters = DataConverters::from(&values);

    assert_eq!(converters.len(), 3);
    assert!(!converters.is_empty());

    let empty: Vec<u8> = Vec::new();
    let converters = DataConverters::from(&empty);

    assert_eq!(converters.len(), 0);
    assert!(converters.is_empty());
}
