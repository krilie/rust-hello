use std::str;

pub fn test() {
    let tao = str::from_utf8(&[0xe9u8, 0x81u8, 0x93u8]).unwrap();
    println!("{}", tao);
    println!("{:?}", tao.chars());
    let tao = '藏';
    println!("{:?}", tao);
    println!("{:?}", tao.is_uppercase());
    println!("{:?}", tao.is_lowercase());
    println!("{:?}", tao.is_alphabetic());
}

pub fn test2() {
    let mut a = String::from("foo工");
    println!("{:p}", a.as_ptr());
    println!("{:p}", &a);
    a.reserve(10);
    println!("{:?}", a.capacity());
}