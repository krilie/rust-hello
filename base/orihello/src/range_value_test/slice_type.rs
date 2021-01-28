pub fn test() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let b = &arr[4..]; // 切片范围
    println!("{:?}", b);
}
