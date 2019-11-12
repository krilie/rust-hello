pub fn test() {
    let x: Result<i32, &str> = Ok(-3);
    let e: Result<i32, &str> = Err("some err");
    println!("{:?}-{:?}", x, e);
}
