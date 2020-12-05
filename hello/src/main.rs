use regex::Regex;

fn main() {
    println!("Hello, world!");
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {}", re.is_match("2020-12-05"));
}

#[test]
pub fn test() {
    println!("unit test of main");
    let rust = "Rust";
    println!("hello,{}!", rust);
}

#[test]
pub fn vvvvv() {
    let mut a1 = 5;
    let a2: i32 = 5;
    assert_eq!(a1, a2);
    let b1: u32 = 5;
}

#[test]
fn one() {
    let mut a: f64 = 1.0;
    let b = 2.0f32;

    //改变 a 的绑定
    a = 2.0;
    println!("{:?}", a);

    //重新绑定为不可变
    let a = a;

    // 不能赋值
    // a = 3.0;

    //类型不匹配
    // assert_eq!(a, b);
}

#[test]
fn let_decode() {
    let (a, mut b): (bool, bool) = (true, false); // 解构
    println!("a = {:?}, b = {:?}", a, b);
    //a 不可变绑定
    //a = false;

    //b 可变绑定
    b = true;
    assert_eq!(a, b);
}

#[test]
fn let_22122() {
// boolean type
    let t = true;
    let f: bool = false;

// char type
    let c = 'c';

// numeric types
    let x = 42;
    let y: u32 = 123_456;
    let z: f64 = 1.23e+2;
    let zero = z.abs_sub(123.4);
    let bin = 0b1111_0000;
    let oct = 0o7320_1546;
    let hex: i64 = 0xf23a_b049;

// string types
    let str = "Hello, world!";
    let mut string = str.to_string();

// arrays and slices
    let a = [0, 1, 2, 3, 4];
    let middle = &a[1..4];
    let mut ten_zeros: [i64; 10] = [0; 10];

// tuples
    let tuple: (i32, &str) = (50, "hello");
    let (fifty, _) = tuple;
    let hello = tuple.1;

// raw pointers
    let x = 5;
    let raw = &x as *const i32;
    let points_at = unsafe { *raw };

    // functions
    fn foo(x: i32) -> i32 { x }
    let bar: fn(i32) -> i32 = foo;
    // explicit conversion
    let decimal = 65.4321_f32;
    let integer = decimal as u8;
    let character = integer as char;

    // type aliases
    type NanoSecond = u64;
    type Point = (u8, u8);
}

#[test]
fn arrays() {
    let mut array: [i32; 3] = [0; 3];
    array[1] = 1;
    array[2] = 2;
    println!("{}", array[0]);
    for x in &array {
        println!("{}", x)
    }
}

#[test]
fn vecc() {
    //创建空Vec
    let v: Vec<i32> = Vec::new();
//使用宏创建空Vec
    let v: Vec<i32> = vec![];
//创建包含5个元素的Vec
    let v = vec![1, 2, 3, 4, 5];
//创建十个零
    let v = vec![0; 10];
//创建可变的Vec，并压入元素3
    let mut v = vec![1, 2];
    v.push(3);
//创建拥有两个元素的Vec，并弹出一个元素
    let mut v = vec![1, 2];
    let two = v.pop();
//创建包含三个元素的可变Vec，并索引一个值和修改一个值
    let mut v = vec![1, 2, 3];
    let three = v[2];
    v[1] = v[1] + 5;
}

#[test]
fn str_test() {
    let hello = "Hello,world";
    let hello: &'static str = "Hello,world";
    println!("{}", hello);
}

#[test]
fn str_2() {
    // 创建一个空的字符串
    let mut s = String::new();
// 从 `&str` 类型转化成 `String` 类型
    let mut hello = String::from("Hello, ");
// 压入字符和压入字符串切片
    hello.push('w');
    hello.push_str("orld!");

// 弹出字符。
    let mut s = String::from("foo");
    assert_eq!(s.pop(), Some('o'));
    assert_eq!(s.pop(), Some('o'));
    assert_eq!(s.pop(), Some('f'));
    assert_eq!(s.pop(), None);
}

#[test]
fn struct_test() {
    // structs
    struct Point {
        x: i32,
        y: i32,
    }
    let point = Point { x: 0, y: 0 };

    // tuple structs
    struct Color(u8, u8, u8);
    let android_green = Color(0xa4, 0xc6, 0x39);
    let Color(red, green, blue) = android_green;

    // A tuple struct’s constructors can be used as functions.
    struct Digit(i32);
    let v = vec![0, 1, 2];
    let d: Vec<Digit> = v.into_iter().map(Digit).collect();

    // newtype: a tuple struct with only one element
    struct Inches(i32);
    let length = Inches(10);
    let Inches(integer_length) = length;

    // unit-like structs
    struct EmptyStruct;
    let empty = EmptyStruct;
}

#[test]
fn ddddd() {
    #[derive(Default)]
    struct Point3d {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point3d::default();
    let point = Point3d { y: 1, ..origin };
    let Point3d { x: x0, y: y0, .. } = point;
}

#[test]
fn stru32() {
    use std::cell::Cell;

    struct Point {
        x: i32,
        y: Cell<i32>,
    }

    let point = Point { x: 5, y: Cell::new(6) };

    point.y.set(7);
}

#[test]
fn graph() {
    mod graph {
        #[derive(Default)]
        pub struct Point {
            pub x: i32,
            y: i32,
        }

        pub fn inside_fn() {
            let p = Point { x: 1, y: 2 };
            println!("{}, {}", p.x, p.y);
        }
    }

    fn outside_fn() {
        let p = graph::Point::default();
        println!("{}", p.x);
        // println!("{}", p.y);
        // field `y` of struct `graph::Point` is private
    }
    outside_fn();
}

#[test]
fn ennnu() {
    // enums
    enum Message {
        Quit,
        ChangeColor(i32, i32, i32),
        Move { x: i32, y: i32 },
        Write(String),
    }

    let x: Message = Message::Move { x: 3, y: 4 };
}

#[test]
fn ifff() {
    let x = 5;
    let y = if x == 5 { 10 } else { 15 };
    println!("{}", y);
}

#[test]
fn whilll() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}


#[test]
fn matchhhh() {
    let day = 5;

    match day {
        0 | 6 => println!("weekend"),
        1...5 => println!("weekday"),
        _ => println!("invalid"),
    }
}

#[test]
fn testmatch2() {
    let x = 1;

    match x {
        e @ 1...5 => println!("got a range element {}", e),
        _ => println!("anything"),
    }
}

#[test]
fn matccccc() {
    let x = 5;
    let mut y = 5;

    match x {
        // the `r` inside the match has the type `&i32`
        ref r => println!("Got a reference to {}", r),
    }

    match y {
        // the `mr` inside the match has the type `&i32` and is mutable
        ref mut mr => println!("Got a mutable reference to {}", mr),
    }
}

#[test]
fn matchhhhh() {
    let pair = (0, -2);

    match pair {
        (0, y) => println!("x is `0` and `y` is `{:?}`", y),
        (x, 0) => println!("`x` is `{:?}` and y is `0`", x),
        _ => println!("It doesn't matter what they are"),
    }
}

#[test]
fn match4444() {
    struct Point {
        x: i32,
        y: i32,
    }

    let origin = Point { x: 0, y: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    enum OptionalInt {
        Value(i32),
        Missing,
    }

    let x = OptionalInt::Value(5);

    match x {
        // 这里是 match 的 if guard 表达式，我们将在以后的章节进行详细介绍
        OptionalInt::Value(i) if i > 5 => println!("Got an int bigger than five!"),
        OptionalInt::Value(..) => println!("Got an int!"),
        OptionalInt::Missing => println!("No such luck."),
    }
}

#[test]
fn letttt() {
    let number = Some(7);
    let mut optional = Some(0);

// If `let` destructures `number` into `Some(i)`, evaluate the block.
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number!");
    }

// While `let` destructures `optional` into `Some(i)`, evaluate the block.
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }
}

#[test]
fn fun_test_o() {
    fn add_one(x: i32) -> i32 {
        x + 1
    }
    println!("{}", add_one(5));
}

#[test]
fn pannnn() {
    fn diverges() -> ! {
        panic!("This function never returns!");
    }
    // diverges();
    let x: i32 = diverges();
    let y: String = diverges();
}

#[test]
fn cloooo() {
    let num = 5;
    let plus_num = |x: i32| x + num;
}

#[test]
fn cloooooooo() {
    let mut num = 5;

    {
        let mut add_num = move |x: i32| num += x;   // 闭包通过move获取了num的所有权

        add_num(5);
    }

// 下面的num在被move之后还能继续使用是因为其实现了Copy特性
// 具体可见所有权(Owership)章节
    assert_eq!(5, num);
}


mod funnnnn {
    fn add_one(x: i32) -> i32 { x + 1 }

    fn apply<F>(f: F, y: i32) -> i32
        where F: Fn(i32) -> i32
    {
        f(y) * y
    }

    fn factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
        Box::new(move |y| x + y)
    }

    #[test]
    fn tmain() {
        let transform: fn(i32) -> i32 = add_one;
        let f0 = add_one(2i32) * 2;
        let f1 = apply(add_one, 2);
        let f2 = apply(transform, 2);
        println!("{}, {}, {}", f0, f1, f2);

        let closure = |x: i32| x + 1;
        let c0 = closure(2i32) * 2;
        let c1 = apply(closure, 2);
        let c2 = apply(|x| x + 1, 2);
        println!("{}, {}, {}", c0, c1, c2);

        let box_fn = factory(1i32);
        let b0 = box_fn(2i32) * 2;
        let b1 = (*box_fn)(2i32) * 2;
        let b2 = (&box_fn)(2i32) * 2;
        println!("{}, {}, {}", b0, b1, b2);

        let add_num = &(*box_fn);
        let translate: &Fn(i32) -> i32 = add_num;
        let z0 = add_num(2i32) * 2;
        let z1 = apply(add_num, 2);
        let z2 = apply(translate, 2);
        println!("{}, {}, {}", z0, z1, z2);
    }
}

mod funcs {
    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }

    impl Circle {
        fn new(x: f64, y: f64, radius: f64) -> Circle {
            Circle {
                x: x,
                y: y,
                radius: radius,
            }
        }

        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }
    }

    #[test]
    fn main() {
        let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };
        println!("{}", c.area());

        // use associated function and method chaining
        println!("{}", Circle::new(0.0, 0.0, 2.0).area());
    }
}

mod ssss {
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

    struct Square {
        x: f64,
        y: f64,
        side: f64,
    }

    impl HasArea for Square {
        fn area(&self) -> f64 {
            self.side * self.side
        }
    }

    fn print_area<T: HasArea>(shape: T) {
        println!("This shape has an area of {}", shape.area());
    }
}

#[test]
fn enum_options() {
    enum Option<T> {
        Some(T),
        None,
    }
    let x: Option<i32> = Option::Some(5);
    let y: Option<f64> = Option::Some(5.0f64);
}


#[test]
fn read_input() {
    use std::io;

    fn read_input() -> io::Result<()> {
        let mut input = String::new();

        io::stdin().read_line(&mut input)?;

        println!("You typed: {}", input.trim());

        Ok(())
    }

    read_input();
}








