pub fn square() -> impl Fn(i32) -> i32 {
    Box::new(|i| i * i)
}
pub fn test_1() {
    let s = square();
    println!("{}", s(45));
}
