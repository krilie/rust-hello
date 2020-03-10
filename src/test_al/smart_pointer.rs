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

mod point3 {
    #[derive(Debug)]
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    use List::{Cons, Nil};
    use std::rc::Rc;

    #[test]
    fn test2() {
        let list = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))))));
        let b = Cons(3, Rc::clone(&list));
        let c = Cons(3, Rc::clone(&list));
        println!("{:?}", list);
        println!("{:?}", b);
        println!("{:?}", c);
    }
}

mod point4 {
    use std::rc::Rc;
    use std::cell::{RefCell, Ref};
    use crate::test_al::smart_pointer::point4::List::{Cons, Nil};

    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }

    #[test]
    fn test_main() {
        let value = Rc::new(RefCell::new(5));
        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
        let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
        *value.borrow_mut() += 10;
        println!("a {:?}", a);
        println!("b {:?}", b);
        println!("c {:?}", c);
    }
}

