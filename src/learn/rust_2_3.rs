//pub fn temp()->i32{
//    return 1;
//}
//pub fn test(){
//    let x=&temp();
//    temp() = *x;
//}

// 显示指定闭包类型
pub fn close_fn_test() {
    let env_var = 1;
    let c: Box<dyn Fn() -> i32> = Box::new(|| env_var + 2);
    println!("{}", c());
}

// 显示指定闭包类型
pub fn close_fn_test2() {
    let env_var = 3;
    let c: Box<dyn Fn() -> i32> = Box::new(|| env_var + 2);
    println!("{}", c());
}

pub fn hello_learn() {
    println!("hello_learn");
}
