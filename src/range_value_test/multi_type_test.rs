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
    pub fn static_show(){
        println!("apple hellll")
    }
    pub fn show(&self) {
        println!("==+++{:?}", self);
    }
}
pub fn test2() {
    let a = Apple {
        name: "aa",
        gender: 32,
    };
    println!("{:?}", a);
    a.show();
    Apple::static_show();
    //++++++++++++++++++++
}
//==================________
#[derive(Debug)]
struct Color(i32,i32,i32);
pub fn test3(){
    let color=Color(1,2,3);
    println!("{:?}-{}-{}",color,color.0,color.2);
    let color2=Color{0:1,1:2,2:30};
    println!("{:?}-{}-{}",color2,color2.0,color2.2)
}
//+++++++++++++============
#[derive(Debug)]
struct Integer(u32);
type Int = i32;
pub fn test4(){
    let int = Integer(32);
    println!("{:?},{}",int,int.0);
    let int2 :Int = 324;
    println!("{}",int2);
}
// 枚举体

// 联合体
