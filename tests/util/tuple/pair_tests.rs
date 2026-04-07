/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Pair Tests
//!
//! Unit tests for the Pair structure.
//!
//! # Author
//!
//! Haixing Hu

use qubit_common::Pair;

#[test]
fn test_new() {
    let pair = Pair::new(1, "hello");
    assert_eq!(pair.first, 1);
    assert_eq!(pair.second, "hello");
}

#[test]
fn test_into_tuple() {
    let pair = Pair::new(1, "hello");
    let (first, second) = pair.into_tuple();
    assert_eq!(first, 1);
    assert_eq!(second, "hello");
}

#[test]
fn test_getters() {
    let pair = Pair::new(1, 2);
    assert_eq!(pair.first(), &1);
    assert_eq!(pair.second(), &2);
}

#[test]
fn test_mutable_getters() {
    let mut pair = Pair::new(1, 2);
    *pair.first_mut() = 10;
    *pair.second_mut() = 20;
    assert_eq!(pair.first, 10);
    assert_eq!(pair.second, 20);
}

#[test]
fn test_map_first() {
    let pair = Pair::new(1, "hello");
    let mapped = pair.map_first(|x| x * 2);
    assert_eq!(mapped.first, 2);
    assert_eq!(mapped.second, "hello");
}

#[test]
fn test_map_second() {
    let pair = Pair::new(1, "hello");
    let mapped = pair.map_second(|s| s.len());
    assert_eq!(mapped.first, 1);
    assert_eq!(mapped.second, 5);
}

#[test]
fn test_swap() {
    let pair = Pair::new(1, "hello");
    let swapped = pair.swap();
    assert_eq!(swapped.first, "hello");
    assert_eq!(swapped.second, 1);
}

#[test]
fn test_from_tuple() {
    let pair: Pair<i32, &str> = (1, "hello").into();
    assert_eq!(pair.first, 1);
    assert_eq!(pair.second, "hello");
}

#[test]
fn test_into_from_pair() {
    let pair = Pair::new(1, "hello");
    let tuple: (i32, &str) = pair.into();
    assert_eq!(tuple, (1, "hello"));
}

#[test]
fn test_display() {
    let pair = Pair::new(1, "hello");
    assert_eq!(format!("{}", pair), "(1, hello)");
}

#[test]
fn test_default() {
    let pair: Pair<i32, i32> = Pair::default();
    assert_eq!(pair.first, 0);
    assert_eq!(pair.second, 0);
}

#[test]
fn test_direct_field_access() {
    let mut pair = Pair::new(1, 2);
    // Test public field access
    pair.first = 100;
    pair.second = 200;
    assert_eq!(pair.first, 100);
    assert_eq!(pair.second, 200);
}

#[test]
fn test_struct_literal_creation() {
    // Test creating Pair using struct literal syntax
    let pair = Pair {
        first: "key",
        second: "value",
    };
    assert_eq!(pair.first, "key");
    assert_eq!(pair.second, "value");
}

#[test]
fn test_pair_with_different_types() {
    let pair1 = Pair::new(42_i32, 2.71_f64);
    assert_eq!(pair1.first, 42);
    assert_eq!(pair1.second, 2.71);

    let pair2 = Pair::new(String::from("hello"), vec![1, 2, 3]);
    assert_eq!(pair2.first, "hello");
    assert_eq!(pair2.second, vec![1, 2, 3]);

    let pair3 = Pair::new(true, Some(100));
    assert!(pair3.first);
    assert_eq!(pair3.second, Some(100));
}

#[test]
fn test_pair_clone() {
    let pair1 = Pair::new(String::from("a"), String::from("b"));
    #[allow(clippy::redundant_clone)]
    let pair2 = pair1.clone();
    assert_eq!(pair1, pair2);
}

#[test]
fn test_pair_copy() {
    let pair1 = Pair::new(1, 2);
    let pair2 = pair1; // Copy happens here
    assert_eq!(pair1, pair2);
    // pair1 is still usable after copy
    assert_eq!(pair1.first, 1);
}

#[test]
fn test_pair_equality() {
    let pair1 = Pair::new(1, 2);
    let pair2 = Pair::new(1, 2);
    let pair3 = Pair::new(1, 3);

    assert_eq!(pair1, pair2);
    assert_ne!(pair1, pair3);
}

#[test]
fn test_pair_hash() {
    use std::collections::HashSet;

    let mut set = HashSet::new();
    set.insert(Pair::new(1, 2));
    set.insert(Pair::new(3, 4));
    set.insert(Pair::new(1, 2)); // Duplicate

    assert_eq!(set.len(), 2);
    assert!(set.contains(&Pair::new(1, 2)));
    assert!(set.contains(&Pair::new(3, 4)));
}
