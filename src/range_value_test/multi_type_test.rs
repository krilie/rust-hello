// 元组
fn move_coords(x: (i32, i32)) -> (i32, i32) {
    (x.0 + 1, x.1 + 2)
}
pub fn test() {
    let tuple: (&'static str, i32, char) = ("hello", 5, 'c');
    println!("{:?}", tuple);
    let coords = (0, 1);
    let result = move_coords(coords);
    println!("{:?}", result);
    let (x, y) = move_coords(coords);
    println!("{}  {}", x, y);
    let (z, _) = move_coords(coords);
    println!("{}", z);
}
// 结构体
#[derive(Debug, PartialEq)]
struct Apple {
    pub name: &'static str,
    pub gender: u32,
}
impl Apple {
    pub fn static_show() {
        println!("apple hellll")
    }
    pub fn show(&self) {
        println!("==+++{:?}", self);
    }
}
pub fn test2() {
    let a = Apple {
        name: "aa",
        gender: 32,
    };
    println!("{:?}", a);
    a.show();
    Apple::static_show();
    //++++++++++++++++++++
}
//==================________
#[derive(Debug)]
struct Color(i32, i32, i32);
pub fn test3() {
    let color = Color(1, 2, 3);
    println!("{:?}-{}-{}", color, color.0, color.2);
    let color2 = Color { 0: 1, 1: 2, 2: 30 };
    println!("{:?}-{}-{}", color2, color2.0, color2.2)
}
//+++++++++++++============
#[derive(Debug)]
struct Integer(u32);
type Int = i32;
pub fn test4() {
    let int = Integer(32);
    println!("{:?},{}", int, int.0);
    let int2: Int = 324;
    println!("{}", int2);
}
//--------------------
#[derive(Debug)]
struct Empty;
pub fn test5() {
    let x = Empty;
    println!("{:?}", x);
}
// 枚举体
#[derive(Debug)]
enum Number {
    #[allow(dead_code)]
    Zero,
    One,
    #[allow(dead_code)]
    Two,
}

#[derive(Debug)]
enum Colors {
    Red = 0xff0000,
    #[allow(dead_code)]
    Green = 0x00ff00,
    #[allow(dead_code)]
    Blue = 0x0000ff,
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

pub fn test6() {
    let a = Number::One;
    match a {
        o @ Number::One => println!("{:?}", o),
        Number::Zero => println!("0"),
        Number::Two => println!("2"),
    }
    let red = Colors::Red;
    println!("{:?}", red);
    let x: fn(u8, u8, u8, u8) -> IpAddr = IpAddr::V4;
    let _: fn(String) -> IpAddr = IpAddr::V6;
    let home = x(127, 0, 0, 1);
    println!("{:?}", home);
}
pub fn test7() {
    let s = &Some("hello".to_string());
    match s {
        Some(x) => println!("{}", x),
        _ => println!("no some"),
    }
}
pub mod coll {

    // 联合体
    // vec vecDeque linkedList
    pub fn test1() {
        let mut v1 = vec![];
        v1.push(1);
        v1.push(2);
        println!("{:?}",v1);
    }
    pub fn test2(){
        use std::collections::VecDeque;
        let mut buf = VecDeque::new();
        buf.push_front(1);
        buf.push_front(2);
        println!("{:?}",buf);
        println!("{:?}",buf.pop_back());
        fn hello(){println!("hello")}
        hello();
    }
    pub fn test_link_list(){
        use std::collections::LinkedList;
        let mut list = LinkedList::new();
        list.push_back('a');
        list.append(&mut list.clone());
        println!("{:?}",list);
    }
    // HashMap BTreeMap
    // HashSet BTreeSet
    // BinaryHeap
}
