fn main() {
    println!("Hello, world!");
}

mod rc_arc {

    #[test]
    fn rc_test(){
        use std::rc::Rc;

        let five = Rc::new(5);
        let five2 = five.clone();
        let five3 = five.clone();
        println!("{} {}",five2,five3);
    }
    #[test]
    fn week_test(){
        use std::rc::Rc;

        let five = Rc::new(5);

        let weak_five = Rc::downgrade(&five);

        let strong_five: Option<Rc<_>> = weak_five.upgrade();
        println!("{:?}", strong_five);
    }



}
