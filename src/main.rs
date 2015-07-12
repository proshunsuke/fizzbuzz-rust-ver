extern crate RustFizzBuzz;

use RustFizzBuzz::rust_fizzbuzz;
use std::env;

fn main() {
    // print from arguments
    let args: Vec<String> = env::args().collect();
    rust_fizzbuzz::print_fizzbuzz(args[1].parse::<i32>().unwrap());
}
