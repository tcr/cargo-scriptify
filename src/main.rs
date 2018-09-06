extern crate failure;
extern crate structopt;

use failure::Error;
use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;
use std::io::prelude::*;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "cargo-scriptify")]
struct Opt {
    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    file: PathBuf,
}

fn main() {
    run().expect("Failed with error");
}

fn run() -> Result<(), Error> {
    let opt = Opt::from_args();

    let mut out = String::new();
    File::open(opt.file)?.read_to_string(&mut out)?;

    println!(r####"#!/usr/bin/env run-cargo-script
//! ```cargo
//! [dependencies]
//! failure = "*"
//! commandspec = "*"
//! ```

#[macro_use]
extern crate commandspec;
extern crate failure;

use failure::Error;

fn main() -> Result<(), Error> {{
    sh_execute!(r###"{}"###)?;
    Ok(())
}}
"####, out);

    Ok(())
}
