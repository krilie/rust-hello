pub mod base_line_2_2;
pub mod channel_test;
pub mod fn_close_param_test;
pub mod fn_close_trait_test;
pub mod fn_return_close;
pub mod fn_test;
pub mod mut_un_mut_2_4;
pub mod rust_2_3;

pub fn test() {
    rust_2_3::hello_learn();
    rust_2_3::close_fn_test();
    rust_2_3::close_fn_test2();
    fn_test::test_1();
    fn_test::test_2();
    fn_test::test_3();
    fn_close_param_test::test_1();
    fn_return_close::test_1();
    fn_return_close::test_2();
    fn_close_trait_test::test_1();
    channel_test::test();
    base_line_2_2::test();
    mut_un_mut_2_4::test();
}
