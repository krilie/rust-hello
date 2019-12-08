mod iter_map;
mod iter_test;
mod iter_collect;

pub fn test() {
    iter_test::test();
    iter_test::test2();
    iter_test::test4();
    iter_map::test();
    iter_map::test2();
    iter_map::test3();
    iter_collect::test();
    iter_collect::test2();
    iter_collect::test3();
    iter_collect::test4();
}
