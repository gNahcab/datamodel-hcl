pub mod cli;

use clap::Parser;
use crate::cli::{Cli, read_in};
fn main() {
    let cli = Cli::parse();
    dbg!(cli);
 //  let _ = match result {
  //     Ok(res) => handle_success(res),
   //    Err(err) => handle_error(err),
 //  };
}

fn handle_success() {
}
fn handle_error() {
}
