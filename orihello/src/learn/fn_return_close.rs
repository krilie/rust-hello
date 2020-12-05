pub fn square() -> impl Fn(i32) -> i32 {
    Box::new(|i| i * i)
}

pub fn square2() -> impl Fn(i32) -> i32 {
    |i| i * i
}

pub fn test_2() {
    let s = square2();
    println!("{}", s(45));
}

pub fn test_1() {
    let s = square();
    println!("{}", s(45));
}
