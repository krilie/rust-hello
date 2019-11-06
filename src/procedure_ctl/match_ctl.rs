pub fn test() {
    let num = 42;
    match num {
        0 => println!("origin"),
        1..=3 => println!("all"),
        5 | 7 | 13 => println!("back luck"),
        n @ 42 => println!("ok {}", n),
        _ => println!("common"),
    }
}

pub fn test2() {
    let boolean = true;
    let mut binary = 0;
    if let true = boolean {
        binary = 1
    }
    println!("{}", binary);
}

pub fn test3() {
    println!("===============");
    let mut v = vec![1, 2, 3, 4];
    loop {
        match v.pop() {
            Some(x) => println!("{}", x),
            None => break,
        }
    }
}
