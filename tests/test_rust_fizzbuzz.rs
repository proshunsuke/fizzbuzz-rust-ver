extern crate RustFizzBuzz;

use RustFizzBuzz::rust_fizzbuzz;

#[test]
fn can_test() {
    assert_eq!(1, 1);
}

#[test]
fn _1is_1() {
    assert_eq!("1", rust_fizzbuzz::fizzbuzz(1));
}

#[test]
fn _3is_fizz() {
    assert_eq!("fizz", rust_fizzbuzz::fizzbuzz(3));
}

#[test]
fn _5is_buzz() {
    assert_eq!("fizz", rust_fizzbuzz::fizzbuzz(3));
}
