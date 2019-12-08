#[test]
pub fn show() {
    println!("ok in test");
}

#[test]
pub fn for_test() {
    for n in 1..=100 {
        println!("{}", n);
    }
}

#[test]
fn while_true() {
    fn while_true(x: i32) -> i32 {
//        while true{
//            return x+1;
//        }
        loop {
            return x + 1;
        }
    }
    let y = while_true(5);
    println!("{}", y);
}

#[test]
fn match_test() {
    let number = 5;
    match number {
        0 => println!("0"),
        a if a > 5 => println!("??"),
        1..=3 => println!("1...3"),
        a @ 4 | a @ 5 | a @ 7 => println!("1 or 5 or 7 real is {}", a),
        _ => println!("unknown"),
    }
}


