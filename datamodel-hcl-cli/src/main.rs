pub mod cli;
mod operations;

use std::env::args;
use clap::Parser;
use crate::cli::{Cli, read_in};
fn main() {
    let cli = read_in();
    println!("cli: {:?}", cli);
    dbg!(cli);
}


