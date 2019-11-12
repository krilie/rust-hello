pub fn test() {
    let str = "Hello Rust";
    let ptr = str.as_ptr();
    let len = str.len();
    println!("{:p} len:{:?} str:{:?}", ptr, len, str);
}

pub fn test2() {
    println!("{:?}", std::mem::size_of::<&[u32; 5]>());
    println!("{:?}", std::mem::size_of::<&mut [u32]>());
}

pub fn test3() {
    enum Void {}
    struct Foo;
    struct Baz {
        foo: Foo,
        qux: (),
        baz: [u8; 0],
    }
    println!("{:?}", std::mem::size_of::<()>());
    println!("{:?}", std::mem::size_of::<Foo>());
    println!("{:?}", std::mem::size_of::<Baz>());
    println!("{:?}", std::mem::size_of::<Void>());
    println!("{:?}", std::mem::size_of::<[(); 10]>());
}

pub fn test4() {
    fn foo() -> ! {
        loop {
            println!("jh")
        }
    }
    let i = if false {
        foo();
    } else {
        100
    };
    println!("{:?}", i);
}

//pub fn test5() {
//    enum Void {}
//    let res: Result<u32, _> = Ok(0);
//    let Ok(num):Result<u32, _>= match res {
//        Ok(num) => Ok(num),
//        _ => Ok(0),
//    };
//    println!("{:?}", num);
//}
