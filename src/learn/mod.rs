pub mod fn_test;
pub mod fn_close_param_test;
pub mod fn_return_close;
pub mod fn_close_trait_test;
pub mod channel_test;
pub mod base_line_2_2;
pub mod rust_2_3;
pub mod mut_un_mut_2_4;

pub fn hello_learn() {
    println!("hello_learn");
}

// 显示指定闭包类型
pub fn close_fn_test() {
    let env_var = 1;
    let c: Box<dyn Fn() -> i32> = Box::new(|| env_var + 2);
    println!("{}",c());
}

// 显示指定闭包类型
pub fn close_fn_test2() {
    let env_var = 3;
    let c: Box<dyn Fn() -> i32> = Box::new(|| env_var + 2);
    println!("{}",c());
}