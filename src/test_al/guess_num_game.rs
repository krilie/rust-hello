use std::io;
use rand::Rng;
#[allow(unused_imports)]
use std::cmp::Ordering;
#[allow(unused_imports)]
use std::thread::yield_now;

pub fn test() {
    println!("Guess the num");
    println!("please input you guess");

    let secret_num = rand::thread_rng().gen_range(1, 101);
    println!("the secret number is: {}", secret_num);

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("failed to read line");
    println!("You guessed: {}", guess);
    // 可由返回值推测出调用那个函数
    let guess: i64 = guess.trim().parse().expect("failure is not num");
    println!("You guess is {}", guess);
}

#[test]
pub fn show_x_y() {
    const PI: u32 = 999;
    println!("pi {}", PI);
    #[allow(unused_variables, unused_mut)] let mut x = 5;
    let y = 10;
    // x = 6;
    let x: i128 = 90;
    println!("x={},y={}", x, y);
    let t: bool = true;
    println!("{}", t);
}

#[test]
pub fn secret_num() {
    loop {
        let secret_num = rand::thread_rng().gen_range(1, 10011);
        println!("the secret number is: {}", secret_num);
        match secret_num.cmp(&34) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}

#[test]
fn test_struct() {
    // 元组类型
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{} {} {}", tup.0, tup.1, tup.2);
    let (_, _, x) = tup; // 元组解构
    println!("{}", x);
}

#[test]
fn test_array() {
    let a = [1, 2, 3, 4, 5, 6, ];
    println!("{}", a.len());
    // 返回值
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("{} {}", y, x);
}

#[test]
fn test_return() {
    fn five() -> i32 { 5 }
    println!("{}", five());
}

#[test]
fn test_control_flow() {
    // if
    let num = 5;
    if num == 5 {
        println!("5")
    } else if num == 6 {
        println!("not 5")
    } else {
        println!("not 6")
    }
    // let if
    let num = if num == 5 { 6 } else { 7 };
    println!("{}", num);
}

#[test]
fn loop_test() {
    // loop while for
    loop {
        println!("again");
        break;
    }
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("{}", result);
}

#[test]
fn while_test() {
    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }
}

#[test]
fn for_test() {
    let a = [10, 20, 23, 24, 56];
    let mut index = 0;
    while index < 5 {
        println!("{}", a[index]);
        index = index + 1;
    }
    for element in a.iter() {
        println!("&i32 {}", element)
    }
    for number in 1..4 {
        println!("readv {}", number);
    }
}

#[test]
fn move_test() {
    let x = 5;
    let y = x; // 移动
    println!("{}", y);

    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s2);
    // println!("{}",s1);
}

#[test]
fn clone_test() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 复制函数 克隆一份
    println!("s1 = {} ; s2 = {}", s1, s2);
    let x = 5;
    let y = x; // 在栈上 克隆一份
    println!("x {} y {}", x, y);
    println!("{}", s1.len());
}

#[allow(dead_code)]
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

#[test]
fn test_mut_str() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
}


#[test]
fn test_slice() {
    #[allow(unused_variables, unused_mut)] let s2 = "hello world";
    let s = String::from("hello world");
    let hello = &s[0..5]; // 切片
    let world = &s[3..10];
    println!("hello {} world {}", hello, world);
}