/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Argument Validation
//!
//! Provides argument validation functionality similar to Java's `Argument` class, but with a design more suitable for Rust conventions.
//!
//! # Module Organization
//!
//! - `error`: Error type definitions
//! - `numeric`: Numeric argument validation
//! - `string`: String argument validation
//! - `collection`: Collection argument validation
//! - `option`: Option argument validation
//! - `condition`: Condition and state validation
//!
//! # Design Philosophy
//!
//! This module uses Rust's trait extension pattern to provide validation methods for various types.
//! Compared to Java's static methods, this approach is more idiomatic in Rust and supports method chaining.
//!
//! # Usage Examples
//!
//! ```rust,ignore
//! use common_rs::lang::argument::{
//!     NumericArgument, StringArgument, CollectionArgument, ArgumentResult
//! };
//! use regex::Regex;
//!
//! fn process_user_input(
//!     age: i32,
//!     username: &str,
//!     tags: &[String],
//! ) -> ArgumentResult<()> {
//!     // Numeric validation
//!     let age = age.require_in_closed_range("age", 0, 150)?;
//!
//!     // String validation (regex + chaining)
//!     let username_pattern = Regex::new(r"^[a-zA-Z][a-zA-Z0-9_]{2,19}$").unwrap();
//!     let username = username
//!         .require_non_blank("username")?
//!         .require_match("username", &username_pattern)?;
//!
//!     // Collection validation (chaining)
//!     let tags = tags
//!         .require_non_empty("tags")?
//!         .require_length_at_most("tags", 10)?;
//!
//!     println!("Age: {}, Username: {}, Tag count: {}", age, username, tags.len());
//!     Ok(())
//! }
//! ```
//!
//! # Author
//!
//! Haixing Hu

pub mod collection;
pub mod condition;
pub mod error;
pub mod numeric;
pub mod option;
pub mod string;

// Re-export main types and traits
pub use collection::{
    require_element_non_null,
    CollectionArgument,
};
pub use condition::{
    check_argument,
    check_argument_fmt,
    check_argument_with_message,
    check_bounds,
    check_element_index,
    check_position_index,
    check_position_indexes,
    check_state,
    check_state_with_message,
};
pub use error::{
    ArgumentError,
    ArgumentResult,
};
pub use numeric::{
    require_equal,
    require_not_equal,
    NumericArgument,
};
pub use option::{
    require_null_or,
    OptionArgument,
};
pub use string::StringArgument;
