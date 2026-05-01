/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
use std::io;

use qubit_common::lang::error::{
    BoxError,
    IntoBoxError,
};

fn convert_error() -> BoxError {
    io::Error::other("converted error").into_box_error()
}

#[test]
fn test_into_box_error_boxes_standard_error() {
    let error = convert_error();

    assert_eq!(error.to_string(), "converted error");
}
