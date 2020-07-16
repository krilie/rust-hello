use std::thread;
use std::time::Duration;

mod borrow_test;
mod comprehensive;
mod fun_closure_iter;
mod iter_test;
mod learn;
mod learn2;
mod procedure_ctl;
mod range_value_test;
mod piston_window_test;
mod trait_test;
mod generics_type;
mod type_sys;
mod new_learn;
mod arch_test;
mod robust_test;
mod thread_test;
mod mata_test;
mod example_learn;
mod learn3;
mod six_funcs;
mod test_al;
mod test_json;
mod test_md5;

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
    robust_test::test();
    thread_test::test();
    mata_test::test();
    // new_learn::test();
    test_al::test();

    thread::spawn(|| {
        for i in 1..4 {
            println!("spawn {}", i);
            thread::sleep(Duration::from_secs(1));
        }
    });
    for i in 1..4 {
        println!("for {}", i);
        thread::sleep(Duration::from_secs(1));
    }


}