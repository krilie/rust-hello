mod box_1 {
    #[test]
    fn test_box() {
        let b = Box::new(5); // 放在栈上的
        println!("{}", b);
    }
}

// mod cons_list {
//     use crate::test_al::smart_pointer::cons_list::List::{Cons, Nil};
//
//     enum List {
//         Cons(i32, List),
//         Nil,
//     }
//
//     #[test]
//     fn test1() {
//         let list = Cons(1, Cons(2, Cons(3, Nil)));
//         println!("{}", list);
//     }
// }

mod point2 {
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use List::{Cons, Nil};

    #[test]
    fn test2() {
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
        println!("{:?}", list);
    }
}