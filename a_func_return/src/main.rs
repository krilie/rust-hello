// main 的完整形式
fn main() -> () {
    println!("Hello, world!");
    let a = 3;
    println!("{}", inc(a));
    let a = [1,2,3,4,5,];
    println!("{}", find(7, &a));
    println!("{}", find(5, &a));
    test();
    return ();
}

fn inc(n: i32) -> i32 { n + 1 } // 有一个返回值的函数

fn find(n: i32, a: &[i32]) -> bool {
    for i in a {
        if *i == n {
            return true;
        }
    }
    false
}

fn diverging() -> ! {
    panic!("This function will never return");
}

fn test(){
    #[derive(Debug)]
    struct Person {
        name: Option<String>,
    }

    let name = "Steve".to_string();
    let x: Option<Person> = Some(Person { name: Some(name) });
    match x {
        Some(Person { name: ref a @ Some(_), .. }) => println!("{:?}", a),
        _ => {}
    }
}
