pub fn test() {
    #[allow(unused_variables)]
    let a = "hello";
    #[allow(unused_variables)]
    let b = "rust";
    #[allow(unused_variables)]
    let c = "world";
    let d = c;
    println!("{:?}", d);
}

pub fn test2() {
    let s = "hello".to_string();
    let join = |i: &str| s + i; // move
    println!("{:?}", join(" world"));
}

pub mod borrow_test {
    fn foo(mut v: [i32; 3]) -> [i32; 3] {
        v[0] = 3;
        v
    }
    fn bar(v:&mut[i32;3]){
        v[0]=3;
    }
    pub fn test(){
        let mut v = [1,2,3];
        foo(v);
        println!("{:?}", v);
        bar(&mut v);
        println!("{:?}", v);
    }
}
