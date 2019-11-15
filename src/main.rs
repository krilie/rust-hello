mod borrow_test;
mod comprehensive;
mod fun_closure_iter;
mod generics_type;
mod iter_test;
mod learn;
mod learn2;
mod piston_window_test;
mod procedure_ctl;
mod range_value_test;
mod trait_test;
mod type_sys;
mod arch_test;

fn main() {
    learn::test();
    learn2::test();
    procedure_ctl::test();
    range_value_test::test();
    //piston_window_test::test();
    generics_type::test();
    type_sys::test();
    comprehensive::test();
    borrow_test::test();
    fun_closure_iter::test();
    trait_test::test();
    iter_test::test();
    arch_test::test();
}
