mod test1 {
    pub trait Summary {
        fn summarize(&self) -> String;
        fn show(&self) {
            println!("这是个默认实现！");
        }
    }

    pub struct Point {
        x: f64,
        y: f64,
    }

    impl Summary for Point {
        fn summarize(&self) -> String {
            (self.x + self.y).to_string()
        }
    }

    // 做为参数
    pub fn show_s(item: impl Summary) {
        item.show();
    }


    #[test]
    fn test_show() {
        let p = Point { x: 0.1, y: 2.3 };
        println!("{:?}", p.summarize());
        p.show();
        show_s(p);
        // p.show(); // p的所有权被转移
    }

    #[test]
    fn test_ee() {
        let v1 = vec![1, 2, 3];
        let total : i32 = v1.iter().sum();
        println!("{}", total);
        let v2 : Vec< _ >= v1.iter().map(|x| x +1).collect();
        for x in v1 {
            println!("{}",x);
        }
        for x in v2 {
            print!("{}-",x)
        }
    }
}