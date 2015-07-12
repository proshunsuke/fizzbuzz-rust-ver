pub fn fizzbuzz(n: i32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => "fizzbuzz".to_string(),
        (_, 0) => "buzz".to_string(),
        (0, _) => "fizz".to_string(),
        _      => n.to_string()
    }
}

pub fn print_fizzbuzz(n: i32) {
    for x in 0..n+1 {
        println!("{}", fizzbuzz(x));
    }
}
