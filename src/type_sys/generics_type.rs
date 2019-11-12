fn foo<T>(x: T) -> T {
    return x;
}
pub fn test() {
    println!("{:?}", foo(1));
    println!("{:?}", foo("hello"));
    println!("{:?}", foo(2.34234f64));
    println!("{:?}", foo(2.34234f32));
}
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x: x, y: y }
    }
}
pub fn test2() {
    let a = Point { x: 1, y: 2 };
    println!("{:?}", a);
    let b = Point { x: 1.1, y: 2f64 };
    println!("{:?}", b);
    let c = Point::new("12", "32");
    println!("{:?}", c);
}
