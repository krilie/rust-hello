fn main() {
    let plus_one = |x: i32| x + 1;

    assert_eq!(2, plus_one(1));

    let plus_two = |x| {
        let mut result: i32 = x;

        result += 1;
        result += 1;

        result
    };

    assert_eq!(4, plus_two(2));

    {
        let plus_one = |x: i32| -> i32 { x + 1 };

        assert_eq!(2, plus_one(1));
    }
    {
        fn plus_one_v1(x: i32) -> i32 { x + 1 }
        let plus_one_v2 = |x: i32| -> i32 { x + 1 };
        let plus_one_v3 = |x: i32| x + 1;
    }
    {
        let mut num = 5;
        let plus_num = |x: i32| x + num;
        assert_eq!(10, plus_num(5)); // 错用
        let y = &mut num;
        *y = 6;
    }

}
