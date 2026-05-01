/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
use std::io;

use qubit_common::lang::error::DynError;

fn error_message(error: &DynError) -> String {
    error.to_string()
}

#[test]
fn test_dyn_error_names_shared_error_object_bounds() {
    let error = io::Error::other("dynamic error");

    assert_eq!(error_message(&error), "dynamic error");
}
