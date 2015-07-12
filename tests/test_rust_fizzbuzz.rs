extern crate RustFizzBuzz;

use RustFizzBuzz::rust_fizzbuzz;

#[test]
fn can_test() {
    assert_eq!(1, 1);
}

#[test]
fn print_1is_1() {
    assert_eq!("1", rust_fizzbuzz::fizzbuzz(1));
}
