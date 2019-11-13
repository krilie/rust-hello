
mod string_test;
mod generics_type;
mod add_generics;
mod drop_test;
mod copy_types;

pub fn test(){
    string_test::test();
    string_test::test2();
    string_test::test3();
    string_test::test4();
//    string_test::test5();
    generics_type::test();
    generics_type::test2();
    generics_type::generics_test::test();
    add_generics::test();
    drop_test::test();
    drop_test::test2();
    copy_types::test();
}