mod func_define;
mod func_mut;
mod func_for_struct;
mod func_ptr_2;
mod closure_test;

pub fn test(){
    func_define::test();
    func_mut::test();
    func_mut::test2();
    func_mut::test3();
    func_mut::test4();
    func_mut::test5();
    func_for_struct::test();
    func_ptr_2::test();
    closure_test::test();
    closure_test::test2();
}