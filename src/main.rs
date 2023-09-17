mod data;
mod scripts;
use std::process;

// This is all boilerplate I picked up somewhere
fn main() {
    if let Err(err) = scripts::run() {
        println!("{}", err);
        process::exit(1);
    }
}
