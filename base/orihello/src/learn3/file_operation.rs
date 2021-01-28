#[test]
fn open_file() {
    // 以只读方式打开文件
    let file = std::fs::File::open("data.txt").unwrap();
    println!("文件打开成功：{:?}", file);
}