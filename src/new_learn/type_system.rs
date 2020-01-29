
#[test]
fn test_type1() {
    let str = "Hello Rust 你";
    let ptr = str.as_ptr();
    let len = str.len();
    println!("{:?}", ptr);
    println!("{:p}", ptr);
    println!("{:?}", len);
}

#[test]
fn test_type2() {
    println!("{:?}", std::mem::size_of::<i32>());
//    let v: () = vec![];
}

#[test]
fn test_for_10() {
    // 高效的循环
    let v: Vec<()> = vec![(); 10];
    for i in v {
        println!("{:?}", i);
    }
}

#[test]
fn tst() {
    fn foo<T>(x: T) -> T {
        return x;
    }
    println!("{:?}", foo(1));
    println!("{:?}", foo("hello"));
    #[derive(Debug)]
    struct Point<T> { x: T, y: T }
    println!("{:?}", Point { x: 1, y: 2 });
    println!("{:?}", Point { x: "hello", y: "world" });
}

#[test]
fn test() {
    use std::ops::Add;
    #[derive(Debug)]
    struct Point(i32, i32);
    impl Add for Point {
        type Output = Point;
        fn add(self, rhs: Point) -> Self::Output {
            Point(self.0 + rhs.0, self.1 + rhs.1)
        }
    }
    println!("{:?}", Point(1, 2) + Point(2, 3));
}

#[test]
pub fn test_trait() {
    trait Show1 {
        fn show(&self) {
            println!("{}", "on show1")
        }
    }
    trait Show2 {
        fn show(&self) {
            println!("{}", "on show2")
        }
    }
    #[warn(dead_code)]
    struct We { age: i32 }
    impl Show1 for We {}
    impl Show2 for We {}
    let i = We { age: 6 };
    Show1::show(&i);
    Show2::show(&i);
}