mod func_pointer;
mod base_types;
mod type_system;
mod thread_test;
mod http_test;

#[allow(dead_code)]
pub fn test() {
    thread_test::test();
    http_test::test_hyper();
}