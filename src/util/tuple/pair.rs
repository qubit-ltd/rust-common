/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Pair
//!
//! A generic pair structure that holds two values of potentially different types.
//!
//! ## Examples
//!
//! ```
//! use qubit_common::Pair;
//!
//! // Compare readability
//! let tuple = ("Alice", 30);
//! println!("Name: {}, Age: {}", tuple.0, tuple.1);  // Less clear
//!
//! let pair = Pair::new("Alice", 30);
//! println!("Name: {}, Age: {}", pair.first, pair.second);  // More clear
//!
//! // Method chaining with Pair
//! let result = Pair::new(5, "hello")
//!     .map_first(|x| x * 2)
//!     .map_second(|s| s.len());
//! assert_eq!(result, Pair::new(10, 5));
//!
//! // Easy conversion between Pair and tuple
//! let tuple = (1, 2);
//! let pair: Pair<i32, i32> = tuple.into();
//! let back_to_tuple: (i32, i32) = pair.into();
//! ```
//!
//! # Author
//!
//! Haixing Hu

use std::fmt;

/// A generic pair structure that holds two values.
///
/// # Type Parameters
///
/// * `F` - The type of the first element
/// * `S` - The type of the second element
///
/// # Pair vs. Tuple
///
/// While Rust's native tuples `(F, S)` can also hold two values, `Pair<F, S>`
/// provides several advantages:
///
/// ## Similarities
///
/// - Both can hold two values of different types
/// - Both support pattern matching and destructuring
/// - Both can be converted to each other via `From`/`Into` traits
///
/// ## Differences
///
/// | Feature | `Pair<F, S>` | `(F, S)` |
/// |---------|-------------|----------|
/// | **Named fields** | ✓ Uses `.first` and `.second` | ✗ Uses `.0` and `.1` |
/// | **Semantic clarity** | ✓ More readable and self-documenting | ✗ Less clear what each element represents |
/// | **Method chaining** | ✓ Provides `map_first()`, `map_second()`, `swap()` | ✗ No built-in transformation methods |
/// | **Mutable access** | ✓ Provides `first_mut()` and `second_mut()` | ✓ Direct field mutation |
/// | **Type aliases** | ✓ Can create meaningful type aliases | ✓ Can create type aliases |
/// | **Pattern matching** | ✓ `Pair { first, second }` | ✓ `(first, second)` |
///
/// ## When to Use `Pair`
///
/// Use `Pair<F, S>` when:
/// - You want more readable code with named fields (`.first`, `.second` vs `.0`, `.1`)
/// - You need transformation methods like `map_first()` or `swap()`
/// - You're building a public API where clarity is important
/// - You want to create semantic type aliases (e.g., `type Coordinate = Pair<f64, f64>`)
///
/// ## When to Use Tuples
///
/// Use native tuples `(F, S)` when:
/// - You need a quick, lightweight data structure for internal use
/// - You're returning multiple values from a function temporarily
/// - You need to match Rust's standard library APIs that use tuples
/// - Performance is critical (though the difference is negligible in practice)
///
/// # Examples
///
/// ```
/// use qubit_common::Pair;
///
/// let pair = Pair::new("name", 42);
/// assert_eq!(pair.first, "name");
/// assert_eq!(pair.second, 42);
///
/// let pair = Pair { first: 1, second: 2.5 };
/// assert_eq!(pair.first, 1);
/// assert_eq!(pair.second, 2.5);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Pair<F, S> {
    /// The first element of the pair
    pub first: F,
    /// The second element of the pair
    pub second: S,
}

impl<F, S> Pair<F, S> {
    /// Creates a new `Pair` with the given values.
    ///
    /// # Arguments
    ///
    /// * `first` - The first element
    /// * `second` - The second element
    ///
    /// # Examples
    ///
    /// ```
    /// use qubit_common::Pair;
    ///
    /// let pair = Pair::new(1, "hello");
    /// assert_eq!(pair.first, 1);
    /// assert_eq!(pair.second, "hello");
    /// ```
    #[inline]
    pub fn new(first: F, second: S) -> Self {
        Pair { first, second }
    }

    /// Consumes the pair and returns a tuple of its elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use qubit_common::Pair;
    ///
    /// let pair = Pair::new(1, "hello");
    /// let (first, second) = pair.into_tuple();
    /// assert_eq!(first, 1);
    /// assert_eq!(second, "hello");
    /// ```
    #[inline]
    pub fn into_tuple(self) -> (F, S) {
        (self.first, self.second)
    }

    /// Returns a reference to the first element.
    ///
    /// # Examples
    ///
    /// ```
    /// use qubit_common::Pair;
    ///
    /// let pair = Pair::new(1, 2);
    /// assert_eq!(pair.first(), &1);
    /// ```
    #[inline]
    pub fn first(&self) -> &F {
        &self.first
    }

    /// Returns a reference to the second element.
    ///
    /// # Examples
    ///
    /// ```
    /// use qubit_common::Pair;
    ///
    /// let pair = Pair::new(1, 2);
    /// assert_eq!(pair.second(), &2);
    /// ```
    #[inline]
    pub fn second(&self) -> &S {
        &self.second
    }

    /// Returns a mutable reference to the first element.
    ///
    /// # Examples
    ///
    /// ```
    /// use qubit_common::Pair;
    ///
    /// let mut pair = Pair::new(1, 2);
    /// *pair.first_mut() = 10;
    /// assert_eq!(pair.first, 10);
    /// ```
    #[inline]
    pub fn first_mut(&mut self) -> &mut F {
        &mut self.first
    }

    /// Returns a mutable reference to the second element.
    ///
    /// # Examples
    ///
    /// ```
    /// use qubit_common::Pair;
    ///
    /// let mut pair = Pair::new(1, 2);
    /// *pair.second_mut() = 20;
    /// assert_eq!(pair.second, 20);
    /// ```
    #[inline]
    pub fn second_mut(&mut self) -> &mut S {
        &mut self.second
    }

    /// Maps the first element to a new value using the provided function.
    ///
    /// # Examples
    ///
    /// ```
    /// use qubit_common::Pair;
    ///
    /// let pair = Pair::new(1, "hello");
    /// let mapped = pair.map_first(|x| x * 2);
    /// assert_eq!(mapped.first, 2);
    /// assert_eq!(mapped.second, "hello");
    /// ```
    #[inline]
    pub fn map_first<F2, Fn>(self, f: Fn) -> Pair<F2, S>
    where
        Fn: FnOnce(F) -> F2,
    {
        Pair {
            first: f(self.first),
            second: self.second,
        }
    }

    /// Maps the second element to a new value using the provided function.
    ///
    /// # Examples
    ///
    /// ```
    /// use qubit_common::Pair;
    ///
    /// let pair = Pair::new(1, "hello");
    /// let mapped = pair.map_second(|s| s.len());
    /// assert_eq!(mapped.first, 1);
    /// assert_eq!(mapped.second, 5);
    /// ```
    #[inline]
    pub fn map_second<S2, Fn>(self, f: Fn) -> Pair<F, S2>
    where
        Fn: FnOnce(S) -> S2,
    {
        Pair {
            first: self.first,
            second: f(self.second),
        }
    }

    /// Swaps the elements of the pair.
    ///
    /// # Examples
    ///
    /// ```
    /// use qubit_common::Pair;
    ///
    /// let pair = Pair::new(1, "hello");
    /// let swapped = pair.swap();
    /// assert_eq!(swapped.first, "hello");
    /// assert_eq!(swapped.second, 1);
    /// ```
    #[inline]
    pub fn swap(self) -> Pair<S, F> {
        Pair {
            first: self.second,
            second: self.first,
        }
    }
}

impl<F, S> From<(F, S)> for Pair<F, S> {
    /// Creates a `Pair` from a tuple.
    ///
    /// # Examples
    ///
    /// ```
    /// use qubit_common::Pair;
    ///
    /// let pair: Pair<i32, &str> = (1, "hello").into();
    /// assert_eq!(pair.first, 1);
    /// assert_eq!(pair.second, "hello");
    /// ```
    #[inline]
    fn from(tuple: (F, S)) -> Self {
        Pair {
            first: tuple.0,
            second: tuple.1,
        }
    }
}

impl<F, S> From<Pair<F, S>> for (F, S) {
    /// Converts a `Pair` into a tuple.
    ///
    /// # Examples
    ///
    /// ```
    /// use qubit_common::Pair;
    ///
    /// let pair = Pair::new(1, "hello");
    /// let tuple: (i32, &str) = pair.into();
    /// assert_eq!(tuple, (1, "hello"));
    /// ```
    #[inline]
    fn from(pair: Pair<F, S>) -> Self {
        (pair.first, pair.second)
    }
}

impl<F: fmt::Display, S: fmt::Display> fmt::Display for Pair<F, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.first, self.second)
    }
}
