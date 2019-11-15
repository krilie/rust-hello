use std::str;
pub fn test() {
    let tao = str::from_utf8(&[0xe9u8, 0x81u8, 0x93u8]).unwrap();
    println!("{}", tao);
    println!("{:?}", tao.chars());
}
