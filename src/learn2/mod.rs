pub mod a2_6;
pub mod fizz_buzz;
pub mod func_ptr;
pub mod learn2_1;
pub mod scope_test;

pub fn test() {
    learn2_1::test();
    a2_6::test();
    println!("{}", fizz_buzz::fizz_buzz(14));
    scope_test::test();
    func_ptr::test();
    func_ptr::test2();
}
