mod thread_test;
mod mutex_test;

pub fn test() {
    thread_test::test();
    thread_test::test2();
    mutex_test::test();
}