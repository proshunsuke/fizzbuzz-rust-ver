pub fn fizzbuzz(n: i32) -> String{
    match n {
        n if n % 3 == 0 => "fizz".to_string(),
        _               => n.to_string()
    }
}
