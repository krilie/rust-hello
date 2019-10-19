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
