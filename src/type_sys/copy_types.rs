pub fn test(){
    let x = 5;
    let y = x; // pod 按位复制
    println!("{:?}", x);
    println!("{:?}", y);
}