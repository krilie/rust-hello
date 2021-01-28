fn main() {
    let mut x: String = String::from("abc");
    {
        let mut some_closure = |c: char| x.push(c);
        some_closure('d');
    }
    println!("x={:?}", x);  //成功打印：x="abcd"
}

#[test]
fn b(){
    let x: Vec<i32> = vec!(1i32, 2, 3);
    let y = &x;
    println!("x={:?}, y={:?}", x, y);
}

#[test]
fn b2(){
    let mut x: i32 = 100;
    {
        let y: &mut i32 = &mut x;
        *y += 2;
    }
    println!("{}", x);
}
