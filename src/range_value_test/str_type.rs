
pub fn test() {
    let truth: &'static str = "一个苹果。。aa";
    let ptr = truth.as_ptr();
    let len = truth.len();
    println!("{:?} {:?}", ptr, len);
    let s = unsafe {
        let slice = std::slice::from_raw_parts(ptr, len);
        std::str::from_utf8(slice)
    };
    println!("{:?}", s);
}
