pub fn test(){
    let str = "Hello Rust";
    let ptr = str.as_ptr();
    let len = str.len();
    println!("{:p} len:{:?} str:{:?}",ptr,len,str);
}