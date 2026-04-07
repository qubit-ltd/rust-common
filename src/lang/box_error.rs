/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Boxed Error Type Alias
//!
//! Provides type aliases for boxed dynamic error types to simplify error
//! handling in generic contexts.
//!
//! # Author
//!
//! Haixing Hu

use std::error::Error;

/// A type alias for `Box<dyn Error + Send + Sync>`.
///
/// This type represents a boxed, dynamically-dispatched error that can be
/// safely sent across thread boundaries and shared between threads.
///
/// # Why Use This Type Alias?
///
/// In Rust, `dyn Error` is a dynamically sized type (DST), which means it
/// cannot be stored directly on the stack or in a struct field. To work with
/// trait objects like `dyn Error`, we must place them behind a pointer, such
/// as `Box`, `Arc`, or `Rc`.
///
/// The type `Box<dyn Error + Send + Sync>` is commonly used throughout the
/// codebase for the following reasons:
///
/// 1. **Type Erasure**: Allows functions to return or store errors of
///    different concrete types without exposing the specific error type.
///
/// 2. **Thread Safety**: The `Send + Sync` bounds ensure the error can be
///    transferred across thread boundaries and shared safely in concurrent
///    contexts.
///
/// 3. **Flexibility**: Enables generic error handling where the exact error
///    type is not known at compile time.
///
/// However, repeatedly writing `Box<dyn Error + Send + Sync>` leads to:
///
/// - **Code Verbosity**: Long type signatures that reduce readability.
/// - **Inconsistency Risk**: Easy to accidentally omit `Send` or `Sync`
///   bounds, leading to compilation errors or thread-safety issues.
/// - **Maintenance Burden**: If we decide to change the error representation
///   (e.g., to `Arc<dyn Error + Send + Sync>`), we would need to update
///   every occurrence.
///
/// By defining a type alias `BoxError`, we:
///
/// - **Reduce Complexity**: Simplify function signatures, struct fields, and
///   generic constraints.
/// - **Ensure Consistency**: Guarantee that all boxed errors have the same
///   bounds (`Send + Sync`).
/// - **Improve Maintainability**: Centralize the definition, making future
///   changes easier.
///
/// # Comparison with Standard Library
///
/// The Rust standard library uses concrete error types rather than type
/// aliases for trait objects:
///
/// - `std::io::Error`: A concrete type that internally uses `Box` to store
///   error details.
/// - `std::fmt::Error`: A zero-sized type (ZST) representing formatting
///   errors.
///
/// These are not trait objects, so they don't require boxing. In contrast,
/// `BoxError` is a type alias for a trait object (`dyn Error`), which must
/// be boxed because it is a DST.
///
/// # When to Use `BoxError`
///
/// Use `BoxError` in the following scenarios:
///
/// 1. **Generic Error Handling**: When a function needs to return errors of
///    different types without exposing the specific error type.
///
/// 2. **Closure Signatures**: To simplify closure signatures that return
///    errors, especially in higher-order functions.
///
/// 3. **Struct Fields**: When storing errors in struct fields where the
///    exact error type is not known at compile time.
///
/// 4. **Trait Implementations**: When implementing traits that require
///    returning a generic error type.
///
/// # When NOT to Use `BoxError`
///
/// Avoid using `BoxError` in the following cases:
///
/// 1. **Public APIs with Specific Errors**: If your public API should expose
///    specific error types (e.g., using `thiserror` or custom error enums),
///    do not use `BoxError`. Concrete error types provide better error
///    handling and debugging experience.
///
/// 2. **Performance-Critical Code**: Boxing introduces a heap allocation. If
///    performance is critical and the error type is known, use a concrete
///    error type instead.
///
/// 3. **Error Recovery**: If the caller needs to match on specific error
///    variants to recover from errors, use a concrete error enum instead of
///    `BoxError`.
///
/// # Examples
///
/// ## Example 1: Simplifying Function Signatures
///
/// ```rust
/// use qubit_common::BoxError;
///
/// // Without BoxError (verbose and error-prone)
/// fn operation_verbose() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
///     Ok("success".to_string())
/// }
///
/// // With BoxError (concise and consistent)
/// fn operation_concise() -> Result<String, BoxError> {
///     Ok("success".to_string())
/// }
/// ```
///
/// ## Example 2: Simplifying Struct Fields
///
/// ```rust
/// use qubit_common::BoxError;
///
/// // Without BoxError
/// struct TaskResult {
///     success: bool,
///     error: Option<Box<dyn std::error::Error + Send + Sync>>,
/// }
///
/// // With BoxError (more readable)
/// struct TaskResultSimple {
///     success: bool,
///     error: Option<BoxError>,
/// }
/// ```
///
/// ## Example 3: Simplifying Closure Signatures
///
/// ```rust
/// use qubit_common::BoxError;
///
/// // Without BoxError (complex generic constraint)
/// fn execute_with_callback<F>(callback: F)
/// where
///     F: FnOnce() -> Result<(), Box<dyn std::error::Error + Send + Sync>>,
/// {
///     let _ = callback();
/// }
///
/// // With BoxError (cleaner constraint)
/// fn execute_with_callback_simple<F>(callback: F)
/// where
///     F: FnOnce() -> Result<(), BoxError>,
/// {
///     let _ = callback();
/// }
/// ```
///
/// ## Example 4: Converting Concrete Errors to BoxError
///
/// ```rust
/// use qubit_common::BoxError;
/// use std::io;
///
/// fn read_file() -> Result<String, BoxError> {
///     let content = std::fs::read_to_string("file.txt")
///         .map_err(|e| Box::new(e) as BoxError)?;
///     Ok(content)
/// }
/// ```
///
/// # Author
///
/// Haixing Hu
pub type BoxError = Box<dyn Error + Send + Sync>;

/// A type alias for `Result<T, BoxError>`.
///
/// This is a convenient shorthand for `Result<T, Box<dyn Error + Send +
/// Sync>>`, commonly used in functions that return a value or a boxed error.
///
/// # Why Use This Type Alias?
///
/// Using `BoxResult<T>` instead of `Result<T, BoxError>` further simplifies
/// function signatures and makes the code more readable. It is particularly
/// useful in codebases where most functions return `Result` with `BoxError`
/// as the error type.
///
/// # When to Use `BoxResult<T>`
///
/// Use `BoxResult<T>` when:
///
/// 1. **Consistent Error Type**: Your function returns a `Result` with
///    `BoxError` as the error type.
///
/// 2. **Simplified Signatures**: You want to reduce the verbosity of
///    function signatures.
///
/// 3. **Codebase Consistency**: You want to maintain a consistent style
///    across your codebase.
///
/// # When NOT to Use `BoxResult<T>`
///
/// Avoid using `BoxResult<T>` when:
///
/// 1. **Specific Error Types**: Your function should return a specific error
///    type for better error handling.
///
/// 2. **Standard Library Compatibility**: You are implementing traits from
///    the standard library that expect `Result<T, E>` with a specific `E`.
///
/// # Examples
///
/// ## Example 1: Simplifying Function Return Types
///
/// ```rust
/// use qubit_common::BoxResult;
///
/// // Without BoxResult
/// fn parse_config() -> Result<Config, Box<dyn std::error::Error + Send + Sync>> {
///     // Implementation
/// #   Ok(Config {})
/// }
///
/// // With BoxResult (more concise)
/// fn parse_config_simple() -> BoxResult<Config> {
///     // Implementation
/// #   Ok(Config {})
/// }
///
/// # struct Config {}
/// ```
///
/// ## Example 2: Using with the `?` Operator
///
/// ```rust
/// use qubit_common::BoxResult;
/// use std::fs;
///
/// fn load_and_parse() -> BoxResult<String> {
///     let content = fs::read_to_string("config.toml")
///         .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
///     Ok(content.to_uppercase())
/// }
/// ```
///
/// ## Example 3: Combining Multiple Error Types
///
/// ```rust
/// use qubit_common::BoxResult;
/// use std::{fs, io};
///
/// fn process_file(path: &str) -> BoxResult<usize> {
///     let content = fs::read_to_string(path)
///         .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
///
///     let parsed: usize = content.trim().parse()
///         .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
///
///     Ok(parsed * 2)
/// }
/// ```
///
/// # Author
///
/// Haixing Hu
pub type BoxResult<T> = Result<T, BoxError>;
