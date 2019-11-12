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
