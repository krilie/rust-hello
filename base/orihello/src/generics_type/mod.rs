mod err_result;
pub mod generics_1;
mod trait_test;

pub fn test() {
    generics_1::test();
    trait_test::test();
    trait_test::debug_trait::test();
    err_result::test();
}
