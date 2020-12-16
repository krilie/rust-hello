fn main() {
    println!("Hello, world!");
}

fn hello(){
    println!("hello");
}

#[test]
#[ignore]
fn it_works(){
    println!("works");
    assert!(false);
}
