# Tuple Module

Generic tuple-like structures with named fields for improved code readability.

## Overview

This module provides two generic structures with named fields:

- **`Pair<F, S>`**: A generic pair structure for storing two values
- **`Triple<F, S, T>`**: A generic triple structure for storing three values

All member fields are `pub` (public) and can be accessed and modified directly.

## Features

### Pair

`Pair<F, S>` is a generic structure that holds two values with the following features:

- **Public fields**: Both `first` and `second` fields are `pub` and directly accessible
- **Flexible creation**: Can be created using the `new()` method or struct literal syntax
- **Type conversion**: Supports bidirectional conversion with tuples `(F, S)`
- **Functional operations**: Provides `map_first`, `map_second`, and `swap` methods
- **Standard traits**: Implements `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash`, `Default`, and `Display`

#### Examples

```rust
use qubit_common::Pair;

// Create using the new method
let pair1 = Pair::new("key", "value");

// Create using struct literal syntax (public fields)
let mut pair2 = Pair {
    first: 1,
    second: 2,
};

// Direct field access and modification
pair2.first = 10;
pair2.second = 20;

// Functional operations
let mapped = pair1.map_first(|k| k.to_uppercase());
let swapped = pair2.swap();

// Conversion to/from tuples
let tuple: (i32, i32) = pair2.into();
let pair3: Pair<i32, i32> = (1, 2).into();
```

### Triple

`Triple<F, S, T>` is a generic structure that holds three values with the following features:

- **Public fields**: All `first`, `second`, and `third` fields are `pub` and directly accessible
- **Flexible creation**: Can be created using the `new()` method or struct literal syntax
- **Type conversion**: Supports bidirectional conversion with tuples `(F, S, T)`
- **Functional operations**: Provides `map_first`, `map_second`, and `map_third` methods
- **Standard traits**: Implements `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash`, `Default`, and `Display`

#### Examples

```rust
use qubit_common::Triple;

// Create using the new method
let triple1 = Triple::new("name", 42, true);

// Create using struct literal syntax (public fields)
let mut triple2 = Triple {
    first: 1,
    second: 2,
    third: 3,
};

// Direct field access and modification
triple2.first = 10;
triple2.second = 20;
triple2.third = 30;

// Functional operations
let mapped = triple1
    .map_first(|s| s.to_uppercase())
    .map_second(|n| n * 2)
    .map_third(|b| !b);

// Conversion to/from tuples
let tuple: (i32, i32, i32) = triple2.into();
let triple3: Triple<i32, i32, i32> = (1, 2, 3).into();
```

## Use Cases

### Typical Use Cases for Pair

1. **Key-value pairs**: Store configuration items or mappings
2. **Coordinates**: Represent 2D coordinates `(x, y)`
3. **Ranges**: Represent numeric ranges `(min, max)`
4. **Result pairs**: Return two related values

### Typical Use Cases for Triple

1. **Records**: Store simple records with three fields
2. **Coordinates**: Represent 3D coordinates `(x, y, z)`
3. **Colors**: Represent RGB color values
4. **Versions**: Represent version numbers `(major, minor, patch)`

## Testing

All tests are located in the `tests/util/tuple/` directory:

- `pair_tests.rs`: Unit tests for the Pair structure
- `triple_tests.rs`: Unit tests for the Triple structure

Run tests:

```bash
# Run all tuple tests
cargo test --test util_tests

# Run documentation tests
cargo test --doc tuple

# Run examples
cargo run --example util_demo
```

## API Reference

For detailed API documentation, please refer to:

- `pair.rs`: Complete implementation of the Pair structure
- `triple.rs`: Complete implementation of the Triple structure

## Author

Haixing Hu

## Copyright

Copyright (c) 2025 - 2026. Haixing Hu, Qubit Co. Ltd. All rights reserved.

