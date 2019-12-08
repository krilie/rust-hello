fn foo<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() % 2 == 0 {
        x
    } else {
        y
    }
}

pub fn test() {
    let x = String::from("hello");
    let z;
    let y = String::from("world");
    z = foo(&x, &y);
    println!("{:?}", z);
}
