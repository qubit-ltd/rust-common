/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Triple Tests
//!
//! Unit tests for the Triple structure.
//!
//! # Author
//!
//! Haixing Hu

use qubit_common::Triple;

#[test]
fn test_new() {
    let triple = Triple::new(1, "hello", true);
    assert_eq!(triple.first, 1);
    assert_eq!(triple.second, "hello");
    assert!(triple.third);
}

#[test]
fn test_into_tuple() {
    let triple = Triple::new(1, "hello", true);
    let (first, second, third) = triple.into_tuple();
    assert_eq!(first, 1);
    assert_eq!(second, "hello");
    assert!(third);
}

#[test]
fn test_getters() {
    let triple = Triple::new(1, 2, 3);
    assert_eq!(triple.first(), &1);
    assert_eq!(triple.second(), &2);
    assert_eq!(triple.third(), &3);
}

#[test]
fn test_mutable_getters() {
    let mut triple = Triple::new(1, 2, 3);
    *triple.first_mut() = 10;
    *triple.second_mut() = 20;
    *triple.third_mut() = 30;
    assert_eq!(triple.first, 10);
    assert_eq!(triple.second, 20);
    assert_eq!(triple.third, 30);
}

#[test]
fn test_map_first() {
    let triple = Triple::new(1, "hello", true);
    let mapped = triple.map_first(|x| x * 2);
    assert_eq!(mapped.first, 2);
    assert_eq!(mapped.second, "hello");
    assert!(mapped.third);
}

#[test]
fn test_map_second() {
    let triple = Triple::new(1, "hello", true);
    let mapped = triple.map_second(|s| s.len());
    assert_eq!(mapped.first, 1);
    assert_eq!(mapped.second, 5);
    assert!(mapped.third);
}

#[test]
fn test_map_third() {
    let triple = Triple::new(1, "hello", true);
    let mapped = triple.map_third(|b| if b { "yes" } else { "no" });
    assert_eq!(mapped.first, 1);
    assert_eq!(mapped.second, "hello");
    assert_eq!(mapped.third, "yes");
}

#[test]
fn test_from_tuple() {
    let triple: Triple<i32, &str, bool> = (1, "hello", true).into();
    assert_eq!(triple.first, 1);
    assert_eq!(triple.second, "hello");
    assert!(triple.third);
}

#[test]
fn test_into_from_triple() {
    let triple = Triple::new(1, "hello", true);
    let tuple: (i32, &str, bool) = triple.into();
    assert_eq!(tuple, (1, "hello", true));
}

#[test]
fn test_display() {
    let triple = Triple::new(1, "hello", true);
    assert_eq!(format!("{}", triple), "(1, hello, true)");
}

#[test]
fn test_default() {
    let triple: Triple<i32, i32, i32> = Triple::default();
    assert_eq!(triple.first, 0);
    assert_eq!(triple.second, 0);
    assert_eq!(triple.third, 0);
}

#[test]
fn test_direct_field_access() {
    let mut triple = Triple::new(1, 2, 3);
    // Test public field access
    triple.first = 100;
    triple.second = 200;
    triple.third = 300;
    assert_eq!(triple.first, 100);
    assert_eq!(triple.second, 200);
    assert_eq!(triple.third, 300);
}

#[test]
fn test_struct_literal_creation() {
    // Test creating Triple using struct literal syntax
    let triple = Triple {
        first: "name",
        second: 42,
        third: true,
    };
    assert_eq!(triple.first, "name");
    assert_eq!(triple.second, 42);
    assert!(triple.third);
}

#[test]
fn test_triple_with_different_types() {
    let triple1 = Triple::new(42_i32, 2.71_f64, 'a');
    assert_eq!(triple1.first, 42);
    assert_eq!(triple1.second, 2.71);
    assert_eq!(triple1.third, 'a');

    let triple2 = Triple::new(String::from("hello"), vec![1, 2, 3], Some(100));
    assert_eq!(triple2.first, "hello");
    assert_eq!(triple2.second, vec![1, 2, 3]);
    assert_eq!(triple2.third, Some(100));

    let triple3 = Triple::new(true, false, true);
    assert!(triple3.first);
    assert!(!triple3.second);
    assert!(triple3.third);
}

#[test]
fn test_triple_clone() {
    let triple1 = Triple::new(String::from("a"), String::from("b"), String::from("c"));
    #[allow(clippy::redundant_clone)]
    let triple2 = triple1.clone();
    assert_eq!(triple1, triple2);
}

#[test]
fn test_triple_copy() {
    let triple1 = Triple::new(1, 2, 3);
    let triple2 = triple1; // Copy happens here
    assert_eq!(triple1, triple2);
    // triple1 is still usable after copy
    assert_eq!(triple1.first, 1);
}

#[test]
fn test_triple_equality() {
    let triple1 = Triple::new(1, 2, 3);
    let triple2 = Triple::new(1, 2, 3);
    let triple3 = Triple::new(1, 2, 4);

    assert_eq!(triple1, triple2);
    assert_ne!(triple1, triple3);
}

#[test]
fn test_triple_hash() {
    use std::collections::HashSet;

    let mut set = HashSet::new();
    set.insert(Triple::new(1, 2, 3));
    set.insert(Triple::new(4, 5, 6));
    set.insert(Triple::new(1, 2, 3)); // Duplicate

    assert_eq!(set.len(), 2);
    assert!(set.contains(&Triple::new(1, 2, 3)));
    assert!(set.contains(&Triple::new(4, 5, 6)));
}

#[test]
fn test_map_all_fields() {
    let triple = Triple::new(1, 2, 3);
    let result = triple
        .map_first(|x| x * 10)
        .map_second(|x| x * 20)
        .map_third(|x| x * 30);

    assert_eq!(result.first, 10);
    assert_eq!(result.second, 40);
    assert_eq!(result.third, 90);
}
