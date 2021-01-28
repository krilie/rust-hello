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

pub mod generics_test {
    #[derive(Debug, PartialEq)]
    struct Foo(i32);

    #[derive(Debug, PartialEq)]
    struct Bar(i32, i32);

    trait Inst {
        fn new(i: i32) -> Self;
    }

    impl Inst for Foo {
        fn new(i: i32) -> Foo {
            Foo(i)
        }
    }

    impl Inst for Bar {
        fn new(i: i32) -> Bar {
            Bar(i, i + 10)
        }
    }

    fn foobar<T: Inst>(i: i32) -> T {
        T::new(i)
    }

    // 根据返回类型推测泛型
    pub fn test() {
        let f: Foo = foobar(10);
        println!("{:?}", f);
        let b: Bar = foobar(20);
        println!("{:?}", b);
    }
}
