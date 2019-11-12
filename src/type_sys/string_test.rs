pub fn test(){
    let str = "Hello Rust";
    let ptr = str.as_ptr();
    let len = str.len();
    println!("{:p} len:{:?} str:{:?}",ptr,len,str);
}

pub fn test2(){
    println!("{:?}", std::mem::size_of::<&[u32;5]>());
    println!("{:?}", std::mem::size_of::<&mut [u32]>());
}