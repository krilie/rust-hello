// 元组
fn move_coords(x: (i32, i32)) -> (i32, i32) {
    (x.0 + 1, x.1 + 2)
}
pub fn test() {
    let tuple: (&'static str, i32, char) = ("hello", 5, 'c');
    println!("{:?}", tuple);
    let coords = (0, 1);
    let result = move_coords(coords);
    println!("{:?}", result);
    let (x, y) = move_coords(coords);
    println!("{}  {}", x, y);
    let (z, _) = move_coords(coords);
    println!("{}", z);
}
// 结构体
#[derive(Debug, PartialEq)]
struct Apple {
    pub name: &'static str,
    pub gender: u32,
}

impl Apple {
    pub fn show(&self){
        println!("==+++{:?}",self);
    }
}

pub fn test2() {
    let a = Apple {
        name: "aa",
        gender: 32,
    };
    println!("{:?}", a);
    a.show();
}
// 枚举体

// 联合体
