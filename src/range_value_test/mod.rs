
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
    multi_type_test::test5();
    multi_type_test::test6();
    multi_type_test::test7();
    multi_type_test::coll::test1();
    multi_type_test::coll::test2();
    multi_type_test::coll::test_link_list();
    multi_type_test::coll::test_hash_map();
    multi_type_test::coll::test_btree_map();
    multi_type_test::coll::test_set();
}
