fn main() {
    let mut v1 = Vec::new();
    v1.push(32);
    println!("{:?}", v1);
    let mut v2 = vec![4;23];
    v2.push(32);
    println!("{:?}", v2);
    // 转换为vector
    let v3: Vec<_> = (1..5).collect();
    println!("{:?}", v3);
    // 随机访问
    let a = vec![1,2,3];
    assert_eq!(a[1],2);
    {
        let v =vec![1, 2, 3];
        assert_eq!(v.get(1), Some(&2));
        assert_eq!(v.get(3), None);
    }
    {
        let mut v = vec![1, 2, 3];
        for i in &v { println!("{}", i); } // 获得引用
        for i in &mut v { println!("{}", i); } // 获得可变引用
        for i in v { println!("{}", i); } // 获得所有权，注意此时Vec的属主将会被转移！！
    }
}
