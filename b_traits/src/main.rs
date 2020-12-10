fn main() {
    let c = Circle {
        x: 0.0,
        y: 0.0,
        radius: 2.0,
    };
    println!("{}", c.area());
    let b = c as Circle;
    println!("{}", b.area());
}

trait HasArea {
    fn area(&self) -> f64;
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}
impl HasArea for i32 {
    fn area(&self) -> f64 {
        6f64
    }
}

mod for_test {
    use std::fmt::Debug;

    fn foo<T: Debug + Clone, B>(s: T, b: B)
        where B: Debug
    {
        s.clone();
        println!("{:?}", s);
    }

    #[test]
    fn test() {
        foo(5, 6);
    }
}

mod for_test2{
    trait Foo {
        fn is_valid(&self) -> bool;
        // 默认方法
        fn is_invalid(&self) -> bool { !self.is_valid() }
    }

    trait Foo2 {
        fn foo(&self);
    }

    trait FooBar : Foo2 {
        fn foobar(&self);
    }

    #[derive(Debug)]
    struct Foo4;

    #[test]
    fn main() {
        println!("{:?}", Foo4);
    }

}
