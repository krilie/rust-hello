#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[test]
fn test_struct() {
    let mut user1 = User {
        username: String::from("aa"),
        email: String::from("bb"),
        sign_in_count: 0,
        active: false,
    };
    user1.username = "aadf".to_string();
    println!("{:?}", user1);
    let user2 = User { active: true, ..user1 };
    println!("{:?}", user2);
}

struct Point(i32, i32, i32);

struct Color;

mod rectangles {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
        fn square(size: u32) -> Rectangle {
            Rectangle { width: size, height: size }
        }
    }

    impl Rectangle {
        fn show(&self) {
            println!("{:?}", self);
        }
    }

    #[test]
    fn test_area() {
        println!("{}", area(23, 32));
        println!("{}", area2(&Rectangle { width: 3, height: 2 }));
        println!("{}", Rectangle { width: 3, height: 2 }.area());
    }

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }

    fn area2(rectangle: &Rectangle) -> u32 {
        rectangle.height * rectangle.width
    }
}

mod enums_test {
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    #[test]
    fn test_enum() {
        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;
        // println!("{}",four);
        // println!("{}",six);
        // let seven = Option::Some("Stringf");
        // println!("{}",seven);
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    #[test]
    fn value_test() {
        let five = Coin::Dime;
        println!("{}", value_in_cents(five));
    }
}

mod vec_test {
    use std::process::id;

    // vector
    #[test]
    fn test_vector() {
        let mut v = Vec::new();
        v.push(32);
        println!("{}", v[0]);
    }

    #[test]
    fn test_v2() {
        let mut v = vec![1, 2, 3];
        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
        for x in &v {
            print!("{} ", x);
        }
        let thrid = &v[3];
        println!("{}", thrid);
        let thrids = v.get(3);
        let thrids = v.get_mut(3);
    }

    #[test]
    fn test2() {
        let mut v = vec![1, 2, 3];
        let first = v[0]; // 不可变借用
        // let first = &v[0]; // 不可变借用
        v.push(6);
        println!("{}", first);
    }

    #[test]
    fn test3() {
        let mut v = vec![1, 2, 3, 4, 5];
        for x in &mut v {
            *x += 50;
        }
        for x in &v {
            print!("{}", x);
        }
    }
}

mod map_hash {
    use std::collections::{HashMap, BTreeMap};

    #[test]
    fn test() {
        let mut s = HashMap::new();
        {
            s.insert(String::from("ok"), "ok");
        }
        s.get("ok");
    }

    #[test]
    fn test2() {
        let mut s = BTreeMap::new();
        {
            s.insert(String::from("ok"), "ok");
            s.insert(String::from("ok"), "ok2"); // 覆盖前一个
        }
        s.get("ok");
        for x in s {
            println!("{}:{}", x.0, x.1);
        }
    }

    #[test]
    fn test3() {
        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];
        let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
        println!("{}", scores.len());
        for (x, y) in scores {
            println!("{} {}", x, y);
        }
    }

    #[test]
    fn test4() {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);
        println!("{:?}", scores);
    }

    #[test]
    fn test5() {
        let text = "hello world wonderful world";
        let mut map = HashMap::new();
        for x in text.split_whitespace() {
            let count = map.entry(x).or_insert(0);
            *count += 1;
        }
        println!("{:?}", map);
    }
}

// 错误处理
mod err_make {
    use std::fs::File;
    use std::io::{Error, ErrorKind};

    #[test]
    fn panic_test() {
        panic!("crash and burn");
    }

    #[test]
    fn panic_test2() {
        let v = vec![1, 2, 3];
        v[99];
    }

    #[test]
    fn panic_file() {
        let f = File::open("hello.txt");
        println!("{:?}", f);
    }

    #[test]
    fn panic_file2() {
        let f = match File::open("hello.txt") {
            Ok(file) => file,
            Err(error) => panic!("error {:?}", error)
        };
    }

    // 创建一个文件
    #[test]
    fn panic_file3() {
        let f = match File::open("hello.txt") {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(file) => file,
                    Err(err) => panic!("{:?}", err)
                },
                other => panic!("{:?}", other)
            }
        };
    }

    #[test]
    fn panic_file4() {
        let f = File::open("hello.txt").unwrap_or_else(|err| {
            if err.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|err| {
                    panic!("{:?}", err);
                })
            } else {
                panic!("{:?}", err);
            }
        });
    }
}


