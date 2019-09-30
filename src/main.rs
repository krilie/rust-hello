fn main() {
    let n = 13f32;
    let big_n = if n < 10f32 && n > -10f32 { 10f32 * n } else { n / 2f32 };
    println!("{}", big_n)
}
