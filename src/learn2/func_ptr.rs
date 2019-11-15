fn math(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    op(a, b)
}
fn sum(a: i32, b: i32) -> i32 {
    a + b
}
fn product(a: i32, b: i32) -> i32 {
    a * b
}
pub fn test() {
    let a = 3;
    let b = 4;
    println!("{}", math(sum, a, b));
    println!("{}", math(product, a, b));
}

// test2
fn is_true() -> bool {
    true
}
fn true_maker() -> fn() -> bool {
    is_true
}
const fn init_len() -> usize {
    5
}
pub fn test2() {
    println!("{} {}", true_maker()(), init_len());
    let arr = [0, init_len()];
    println!("{}", arr.len());
}
