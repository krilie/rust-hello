pub fn test_1(){
    let s = "hello";
    let c = ||{println!("{:?}",s)};
    c();
    c();
    println!("{:?}",s);
}