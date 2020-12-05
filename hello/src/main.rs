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
fn one(){
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
fn let_decode(){
    let (a, mut b): (bool,bool) = (true, false); // 解构
    println!("a = {:?}, b = {:?}", a, b);
    //a 不可变绑定
    //a = false;

    //b 可变绑定
    b = true;
    assert_eq!(a, b);
}

#[test]
fn let_22122(){
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
    let mut array: [i32;3] = [0;3];
    array[1] = 1;
    array[2] = 2;
    println!("{}",array[0]);
    for x in &array {
        println!("{}",x)
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
    println!("{}",hello);
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
            let p = Point {x:1, y:2};
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
