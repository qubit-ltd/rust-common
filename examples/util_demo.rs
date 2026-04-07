/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Util Demo
//!
//! Demonstrates the usage of Pair and Triple utility types.
//!
//! # Author
//!
//! Haixing Hu

use qubit_common::{
    Pair,
    Triple,
};

fn main() {
    println!("=== Pair Demo ===\n");

    // Create Pair using new method
    let pair1 = Pair::new("name", "Alice");
    println!("pair1 (using new): {}", pair1);

    // Create Pair using struct literal (public fields)
    let mut pair2 = Pair {
        first: 1,
        second: 100,
    };
    println!("pair2 (using struct literal): {}", pair2);

    // Direct field access (public fields)
    pair2.first = 42;
    pair2.second = 200;
    println!("pair2 (after modification): {}", pair2);

    // Using methods
    let pair3 = Pair::new(10, 20);
    println!("pair3: {}", pair3);
    println!("  first: {}, second: {}", pair3.first(), pair3.second());

    // Map operations
    let pair4 = pair3.map_first(|x| x * 2).map_second(|x| x * 3);
    println!("pair4 (after mapping): {}", pair4);

    // Swap
    let pair5 = Pair::new("key", 123);
    let swapped = pair5.swap();
    println!("pair5: {}, swapped: {}", pair5, swapped);

    // Convert to tuple
    let (f, s) = pair5.into_tuple();
    println!("Converted to tuple: ({}, {})", f, s);

    println!("\n=== Triple Demo ===\n");

    // Create Triple using new method
    let triple1 = Triple::new("Alice", 30, true);
    println!("triple1 (using new): {}", triple1);

    // Create Triple using struct literal (public fields)
    let mut triple2 = Triple {
        first: 1,
        second: 2,
        third: 3,
    };
    println!("triple2 (using struct literal): {}", triple2);

    // Direct field access (public fields)
    triple2.first = 10;
    triple2.second = 20;
    triple2.third = 30;
    println!("triple2 (after modification): {}", triple2);

    // Using methods
    let triple3 = Triple::new(100, 200, 300);
    println!("triple3: {}", triple3);
    println!(
        "  first: {}, second: {}, third: {}",
        triple3.first(),
        triple3.second(),
        triple3.third()
    );

    // Map operations
    let triple4 = triple3
        .map_first(|x| x / 10)
        .map_second(|x| x / 10)
        .map_third(|x| x / 10);
    println!("triple4 (after mapping): {}", triple4);

    // Convert to tuple
    let (f, s, t) = triple4.into_tuple();
    println!("Converted to tuple: ({}, {}, {})", f, s, t);

    println!("\n=== Advanced Usage ===\n");

    // Pair with different types
    let config = Pair {
        first: "database_url",
        second: "postgresql://localhost/mydb",
    };
    println!("Config: {}", config);

    // Triple as a record
    let record = Triple {
        first: "user_001",
        second: "Alice",
        third: 25,
    };
    println!(
        "User Record: ID={}, Name={}, Age={}",
        record.first, record.second, record.third
    );

    // Nested structures
    let nested = Pair {
        first: Pair::new(1, 2),
        second: Triple::new(3, 4, 5),
    };
    println!(
        "Nested: pair=({}, {}), triple=({}, {}, {})",
        nested.first.first,
        nested.first.second,
        nested.second.first,
        nested.second.second,
        nested.second.third
    );
}
