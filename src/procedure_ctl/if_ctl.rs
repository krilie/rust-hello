pub fn test() {
    let n = 9;
    let big_n = if n < 10 && n > -10 { 10 * n } else { n / 2 };
    println!("{}", big_n)
}
