
pub fn test(){
    let x = 5;
    let y = x; // pod 按位复制
    println!("{:?}", x);
    println!("{:?}", y);
}

pub fn test2(){
    let x = Box::new(5);
    let y = x.clone(); // x.clone(); 借用
    println!("{:?}", x);
    println!("{:?}", y);
}

pub fn test3(){
    #[derive(Debug,Copy,Clone)]
    struct A{
        a:i32,b:u32,
    }
    let a = A{a:1,b:2};
    let b = a;
    println!("{:?}", a);
    println!("{:?}", b);
}