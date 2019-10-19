fn call<F>(closure: F) -> i32
where
    F: Fn(i32) -> i32,
{
    return closure(4);
}
fn counter(i: i32) -> i32 {
    i + 1
}

pub fn test_1() {
    println!("{:?}", call(counter));
}
