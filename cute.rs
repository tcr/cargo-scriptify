#!/usr/bin/env run-cargo-script
//! ```cargo
//! [dependencies]
//! failure = "*"
//! commandspec = "*"
//! ```

#[macro_use]
extern crate commandspec;
extern crate failure;

use failure::Error;

fn main() -> Result<(), Error> {
    sh_execute!(r###"#!/bin/bash

echo hi
"###)?;
    Ok(())
}

