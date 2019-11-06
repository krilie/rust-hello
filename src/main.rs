mod learn;
mod learn2;
mod procedure_ctl;

fn main() {
    learn::hello_learn();
    learn::close_fn_test();
    learn::close_fn_test2();
    learn::fn_test::test_1();
    learn::fn_test::test_2();
    learn::fn_test::test_3();
    learn::fn_close_param_test::test_1();
    learn::fn_return_close::test_1();
    learn::fn_return_close::test_2();
    learn::fn_close_trait_test::test_1();
    learn::channel_test::test();
    learn::base_line_2_2::test();
//    learn::rust_2_3::test(); // error[E0070]: invalid left-hand side expression
    learn::mut_un_mut_2_4::test();
    // test learn 2
    learn2::test();
    procedure_ctl::test();
}