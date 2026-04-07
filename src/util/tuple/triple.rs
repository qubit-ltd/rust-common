/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Triple
//!
//! A generic triple structure that holds three values of potentially different types.
//!
//! # Author
//!
//! Haixing Hu

use std::fmt;

/// A generic triple structure that holds three values.
///
/// # Type Parameters
///
/// * `F` - The type of the first element
/// * `S` - The type of the second element
/// * `T` - The type of the third element
///
/// # Comparison with Native Tuples
///
/// While Rust's native tuples `(F, S, T)` and `Triple<F, S, T>` both store three values,
/// they differ in several key aspects:
///
/// ## Similarities
///
/// - Both can hold three values of different types
/// - Both support pattern matching and destructuring
/// - Both can be converted to each other via `From`/`Into` traits
/// - Both have similar memory layout and zero runtime overhead
///
/// ## Differences
///
/// | Feature | Native Tuple `(F, S, T)` | `Triple<F, S, T>` |
/// |---------|-------------------------|-------------------|
/// | Field Access | `.0`, `.1`, `.2` (positional) | `.first`, `.second`, `.third` (named) |
/// | Semantics | Anonymous, positional | Self-documenting, semantic |
/// | Methods | Limited built-in methods | Rich API (map, getters, setters) |
/// | Readability | Less clear in complex code | More expressive and maintainable |
/// | Type Alias | Hard to create meaningful aliases | Easy to create domain-specific types |
///
/// ## When to Use `Triple` vs Native Tuples
///
/// **Use `Triple` when:**
/// - Field names improve code readability (e.g., `triple.first` vs `tuple.0`)
/// - You need additional methods like `map_first()`, `map_second()`, etc.
/// - Creating domain-specific types (e.g., `type Coordinate = Triple<f64, f64, f64>`)
/// - The triple represents a logical entity with semantic meaning
/// - You want to implement custom traits or behaviors
///
/// **Use native tuples when:**
/// - You need quick, temporary grouping of values
/// - Working with existing APIs that expect tuples
/// - Pattern matching is the primary use case
/// - Minimal overhead and simplicity are paramount
///
/// # Examples
///
/// ```
/// use qubit_common::Triple;
///
/// let triple = Triple::new("name", 42, true);
/// assert_eq!(triple.first, "name");
/// assert_eq!(triple.second, 42);
/// assert_eq!(triple.third, true);
///
/// let triple = Triple { first: 1, second: 2.5, third: "hello" };
/// assert_eq!(triple.first, 1);
/// assert_eq!(triple.second, 2.5);
/// assert_eq!(triple.third, "hello");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Triple<F, S, T> {
    /// The first element of the triple
    pub first: F,
    /// The second element of the triple
    pub second: S,
    /// The third element of the triple
    pub third: T,
}

impl<F, S, T> Triple<F, S, T> {
    /// Creates a new `Triple` with the given values.
    ///
    /// # Arguments
    ///
    /// * `first` - The first element
    /// * `second` - The second element
    /// * `third` - The third element
    ///
    /// # Examples
    ///
    /// ```
    /// use qubit_common::Triple;
    ///
    /// let triple = Triple::new(1, "hello", true);
    /// assert_eq!(triple.first, 1);
    /// assert_eq!(triple.second, "hello");
    /// assert_eq!(triple.third, true);
    /// ```
    #[inline]
    pub fn new(first: F, second: S, third: T) -> Self {
        Triple {
            first,
            second,
            third,
        }
    }

    /// Consumes the triple and returns a tuple of its elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use qubit_common::Triple;
    ///
    /// let triple = Triple::new(1, "hello", true);
    /// let (first, second, third) = triple.into_tuple();
    /// assert_eq!(first, 1);
    /// assert_eq!(second, "hello");
    /// assert_eq!(third, true);
    /// ```
    #[inline]
    pub fn into_tuple(self) -> (F, S, T) {
        (self.first, self.second, self.third)
    }

    /// Returns a reference to the first element.
    ///
    /// # Examples
    ///
    /// ```
    /// use qubit_common::Triple;
    ///
    /// let triple = Triple::new(1, 2, 3);
    /// assert_eq!(triple.first(), &1);
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
    /// use qubit_common::Triple;
    ///
    /// let triple = Triple::new(1, 2, 3);
    /// assert_eq!(triple.second(), &2);
    /// ```
    #[inline]
    pub fn second(&self) -> &S {
        &self.second
    }

    /// Returns a reference to the third element.
    ///
    /// # Examples
    ///
    /// ```
    /// use qubit_common::Triple;
    ///
    /// let triple = Triple::new(1, 2, 3);
    /// assert_eq!(triple.third(), &3);
    /// ```
    #[inline]
    pub fn third(&self) -> &T {
        &self.third
    }

    /// Returns a mutable reference to the first element.
    ///
    /// # Examples
    ///
    /// ```
    /// use qubit_common::Triple;
    ///
    /// let mut triple = Triple::new(1, 2, 3);
    /// *triple.first_mut() = 10;
    /// assert_eq!(triple.first, 10);
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
    /// use qubit_common::Triple;
    ///
    /// let mut triple = Triple::new(1, 2, 3);
    /// *triple.second_mut() = 20;
    /// assert_eq!(triple.second, 20);
    /// ```
    #[inline]
    pub fn second_mut(&mut self) -> &mut S {
        &mut self.second
    }

    /// Returns a mutable reference to the third element.
    ///
    /// # Examples
    ///
    /// ```
    /// use qubit_common::Triple;
    ///
    /// let mut triple = Triple::new(1, 2, 3);
    /// *triple.third_mut() = 30;
    /// assert_eq!(triple.third, 30);
    /// ```
    #[inline]
    pub fn third_mut(&mut self) -> &mut T {
        &mut self.third
    }

    /// Maps the first element to a new value using the provided function.
    ///
    /// # Examples
    ///
    /// ```
    /// use qubit_common::Triple;
    ///
    /// let triple = Triple::new(1, "hello", true);
    /// let mapped = triple.map_first(|x| x * 2);
    /// assert_eq!(mapped.first, 2);
    /// assert_eq!(mapped.second, "hello");
    /// assert_eq!(mapped.third, true);
    /// ```
    #[inline]
    pub fn map_first<F2, Fn>(self, f: Fn) -> Triple<F2, S, T>
    where
        Fn: FnOnce(F) -> F2,
    {
        Triple {
            first: f(self.first),
            second: self.second,
            third: self.third,
        }
    }

    /// Maps the second element to a new value using the provided function.
    ///
    /// # Examples
    ///
    /// ```
    /// use qubit_common::Triple;
    ///
    /// let triple = Triple::new(1, "hello", true);
    /// let mapped = triple.map_second(|s| s.len());
    /// assert_eq!(mapped.first, 1);
    /// assert_eq!(mapped.second, 5);
    /// assert_eq!(mapped.third, true);
    /// ```
    #[inline]
    pub fn map_second<S2, Fn>(self, f: Fn) -> Triple<F, S2, T>
    where
        Fn: FnOnce(S) -> S2,
    {
        Triple {
            first: self.first,
            second: f(self.second),
            third: self.third,
        }
    }

    /// Maps the third element to a new value using the provided function.
    ///
    /// # Examples
    ///
    /// ```
    /// use qubit_common::Triple;
    ///
    /// let triple = Triple::new(1, "hello", true);
    /// let mapped = triple.map_third(|b| if b { "yes" } else { "no" });
    /// assert_eq!(mapped.first, 1);
    /// assert_eq!(mapped.second, "hello");
    /// assert_eq!(mapped.third, "yes");
    /// ```
    #[inline]
    pub fn map_third<T2, Fn>(self, f: Fn) -> Triple<F, S, T2>
    where
        Fn: FnOnce(T) -> T2,
    {
        Triple {
            first: self.first,
            second: self.second,
            third: f(self.third),
        }
    }
}

impl<F, S, T> From<(F, S, T)> for Triple<F, S, T> {
    /// Creates a `Triple` from a tuple.
    ///
    /// # Examples
    ///
    /// ```
    /// use qubit_common::Triple;
    ///
    /// let triple: Triple<i32, &str, bool> = (1, "hello", true).into();
    /// assert_eq!(triple.first, 1);
    /// assert_eq!(triple.second, "hello");
    /// assert_eq!(triple.third, true);
    /// ```
    #[inline]
    fn from(tuple: (F, S, T)) -> Self {
        Triple {
            first: tuple.0,
            second: tuple.1,
            third: tuple.2,
        }
    }
}

impl<F, S, T> From<Triple<F, S, T>> for (F, S, T) {
    /// Converts a `Triple` into a tuple.
    ///
    /// # Examples
    ///
    /// ```
    /// use qubit_common::Triple;
    ///
    /// let triple = Triple::new(1, "hello", true);
    /// let tuple: (i32, &str, bool) = triple.into();
    /// assert_eq!(tuple, (1, "hello", true));
    /// ```
    #[inline]
    fn from(triple: Triple<F, S, T>) -> Self {
        (triple.first, triple.second, triple.third)
    }
}

impl<F: fmt::Display, S: fmt::Display, T: fmt::Display> fmt::Display for Triple<F, S, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.first, self.second, self.third)
    }
}
