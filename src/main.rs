// SPDX-FileCopyrightText: © 2024 Matt Williams <matt.williams@bristol.ac.uk>
// SPDX-License-Identifier: MIT

use clap::Parser;

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[derive(Parser)]
#[command(version = built_info::GIT_VERSION.expect("Could not find version"))]
struct Args {}

fn main() {
    let _ = Args::parse();
}
