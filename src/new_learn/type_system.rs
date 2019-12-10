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
    println!("{:?}", std::mem::size_of::<(i32)>());
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
