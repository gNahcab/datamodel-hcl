pub mod cli;
use crate::cli::read_in;
fn main() {
    let cli = read_in();
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
