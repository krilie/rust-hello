pub fn test_1() {
    let s = "hello";
    let c = || println!("{:?}", s);
    c();
    c();
    println!("{:?}", s);
}

pub fn test_2() {
    let s = "hello";
    let c = move || println!("{:?}", s);
    c();
    c();
    println!("{:?}", s);
}

pub fn test_3() {
    let mut c: Vec<Box<dyn Fn()>> = vec![];
    boxed_close(&mut c);
    for f in c {
        f();
    }
}

fn boxed_close(c: &mut Vec<Box<dyn Fn()>>) {
    let s = "second";
    c.push(Box::new(|| println!("first")));
    c.push(Box::new(move || println!("{}", s)));
    c.push(Box::new(|| println!("third")));
}
