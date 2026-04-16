/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
use qubit_common::{
    require_equal,
    require_not_equal,
    NumericArgument,
};

#[test]
fn zero_and_non_zero() {
    assert!(0i32.require_zero("v").is_ok());
    assert!(1i32.require_zero("v").is_err());
    assert!(0u32.require_non_zero("v").is_err());
    assert!(5u32.require_non_zero("v").is_ok());
    assert!(0.0f64.require_zero("v").is_ok());
    assert!((-0.0f64).require_non_zero("v").is_err());
}

#[test]
fn sign_related_checks() {
    // positive / non-negative
    assert!(10i32.require_positive("a").is_ok());
    assert!(0i32.require_positive("a").is_err());
    assert!((-1i32).require_positive("a").is_err());
    assert!(0i32.require_non_negative("a").is_ok());
    assert!((-1i32).require_non_negative("a").is_err());

    // negative / non-positive
    assert!((-5i32).require_negative("b").is_ok());
    assert!(0i32.require_negative("b").is_err());
    assert!(5i32.require_negative("b").is_err());
    assert!(0i32.require_non_positive("b").is_ok());
    assert!((5i32).require_non_positive("b").is_err());
}

#[test]
fn range_checks_closed_open_variants_with_integers() {
    // closed [min,max]
    assert!(5i32.require_in_closed_range("x", 1, 10).is_ok());
    assert!(1i32.require_in_closed_range("x", 1, 10).is_ok());
    assert!(10i32.require_in_closed_range("x", 1, 10).is_ok());
    assert!(0i32.require_in_closed_range("x", 1, 10).is_err());
    assert!(11i32.require_in_closed_range("x", 1, 10).is_err());

    // open (min,max)
    assert!(5i32.require_in_open_range("x", 1, 10).is_ok());
    assert!(1i32.require_in_open_range("x", 1, 10).is_err());
    assert!(10i32.require_in_open_range("x", 1, 10).is_err());

    // left-open (min,max]
    assert!(5i32.require_in_left_open_range("x", 1, 10).is_ok());
    assert!(1i32.require_in_left_open_range("x", 1, 10).is_err());
    assert!(10i32.require_in_left_open_range("x", 1, 10).is_ok());

    // right-open [min,max)
    assert!(5i32.require_in_right_open_range("x", 1, 10).is_ok());
    assert!(1i32.require_in_right_open_range("x", 1, 10).is_ok());
    assert!(10i32.require_in_right_open_range("x", 1, 10).is_err());
}

#[test]
fn range_checks_min_greater_than_max_should_fail() {
    assert!(5i32.require_in_closed_range("x", 10, 1).is_err());
    assert!(5i32.require_in_open_range("x", 10, 1).is_err());
    assert!(5i32.require_in_left_open_range("x", 10, 1).is_err());
    assert!(5i32.require_in_right_open_range("x", 10, 1).is_err());
}

#[test]
fn comparison_checks() {
    assert!(5i32.require_less("x", 6).is_ok());
    assert!(5i32.require_less("x", 5).is_err());
    assert!(5i32.require_less_equal("x", 5).is_ok());
    assert!(6i32.require_less_equal("x", 5).is_err());

    assert!(6i32.require_greater("x", 5).is_ok());
    assert!(5i32.require_greater("x", 5).is_err());
    assert!(5i32.require_greater_equal("x", 5).is_ok());
    assert!(4i32.require_greater_equal("x", 5).is_err());
}

#[test]
fn free_functions_equal_and_not_equal() {
    assert!(require_equal("a", 100, "b", 100).is_ok());
    let err1 = require_equal("a", 100, "b", 200).unwrap_err();
    assert!(err1.message().contains("a"));

    assert!(require_not_equal("a", 1, "b", 2).is_ok());
    let err2 = require_not_equal("a", 2, "b", 2).unwrap_err();
    assert!(err2.message().contains("cannot be equal"));
}

#[test]
fn float_edge_cases_nan_and_infinite() {
    let nan = f64::NAN;
    assert!(nan.require_in_closed_range("f", -1.0, 1.0).is_err());
    assert!(nan.require_less("f", 0.0).is_err());
    assert!(nan.require_greater("f", 0.0).is_err());
    assert!(nan.require_zero("f").is_err());
    assert!(nan.require_non_zero("f").is_err());

    let inf = f64::INFINITY;
    assert!(inf.require_less("f", f64::MAX).is_err());
    assert!(inf.require_greater_equal("f", 0.0).is_ok());

    let ninf = f64::NEG_INFINITY;
    assert!(ninf.require_greater("f", f64::MIN).is_err());
    assert!(ninf.require_less_equal("f", 0.0).is_ok());
}

#[test]
fn float_range_bounds_nan_should_fail() {
    assert!(1.0f64.require_in_closed_range("f", f64::NAN, 1.0).is_err());
    assert!(1.0f64.require_in_closed_range("f", -1.0, f64::NAN).is_err());
    assert!(1.0f64.require_in_open_range("f", f64::NAN, 1.0).is_err());
    assert!(1.0f64.require_in_open_range("f", -1.0, f64::NAN).is_err());
}

#[test]
fn big_integer_edges() {
    let max = i128::MAX;
    let min = i128::MIN;
    assert!(max.require_in_closed_range("v", min, max).is_ok());
    assert!(min.require_in_closed_range("v", min, max).is_ok());

    let umax = u128::MAX;
    assert!(umax.require_greater_equal("u", 0u128).is_ok());
}
