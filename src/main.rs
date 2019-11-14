mod learn;
mod learn2;
mod procedure_ctl;
mod range_value_test;
mod piston_window_test;
mod trait_test;
mod generics_type;
mod type_sys;
mod comprehensive;
mod borrow_test;
mod fun_closure_iter;

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
}