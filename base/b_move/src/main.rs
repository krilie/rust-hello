fn main() {
    let a: String = String::from("xyz");
    let b = a;
    println!("{}", b);
    // println!("{}", a);
    {
        let a: i32 = 100;
        let b = a;
        println!("{}", a);
    }
    {
        let a: String = String::from("xyz");
        let b = a.clone();  // <-注意此处的clone
        println!("{}", b);
    }
    {
        let a = vec![1, 2, 3];  //不可变绑定, a <=> 内存区域A(1,2,3)
        let mut a = a;  //可变绑定， a <=> 内存区域A(1,2,3), 注意此a已非上句a，只是名字一样而已
        a.push(4);
        println!("{:?}", a);  //打印：[1, 2, 3, 4]
    }
}
