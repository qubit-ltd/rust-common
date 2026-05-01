/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
use std::io;

use qubit_common::lang::error::BoxResult;

fn fail_with_question_mark() -> BoxResult<()> {
    Err(io::Error::other("box result failure"))?;
    Ok(())
}

#[test]
fn test_box_result_accepts_question_mark_conversion() {
    let error = fail_with_question_mark().expect_err("io error should be boxed");

    assert_eq!(error.to_string(), "box result failure");
}
