mod range_test;
mod slice_type;
mod str_type;
mod multi_type_test;

pub fn test() {
    range_test::test();
    slice_type::test();
    str_type::test();
    multi_type_test::test();
    multi_type_test::test2();
    multi_type_test::test3();
    multi_type_test::test4();
}
