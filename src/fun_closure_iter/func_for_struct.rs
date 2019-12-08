fn hello() {
    println!("hello function pointer");
}

pub fn test() {
    let fn_ptr: fn() = hello;
    println!("{:p}", fn_ptr);
    fn_ptr();
    let other_fn = hello;
    other_fn();
}
