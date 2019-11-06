pub fn fizz_buzz(num: i32) -> String {
    if num % 15 == 0 {
        return "fizzbuzz".to_string();
    }
    return "n".to_string();
}
