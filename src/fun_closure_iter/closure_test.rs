fn counter(i: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |n: i32| n + i)
}
pub fn test() {
    let f = counter(3);
    println!("{:?}", f(5));
}

pub fn test2() {
    let add = |a: i32, b: i32| -> i32 { a + b };
    println!("{:?}", add(3, 4));
}
