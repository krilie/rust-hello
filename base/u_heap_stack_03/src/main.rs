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

    mod arc_test{
        use std::sync::Arc;
        use std::thread;
        use std::thread::sleep;
        use std::time::Duration;

        #[test]
        fn main() {
            let numbers: Vec<_> = (0..100u32).collect();
            let shared_numbers = Arc::new(numbers);
            let mut joins = vec![];
            for _ in 0..10 {
                let child_numbers = shared_numbers.clone();

                let join = thread::spawn(move || {
                    let local_numbers = &child_numbers[..];
                    println!("{:?}", local_numbers);
                    // Work with the local numbers
                });
                joins.push(join)
            }
            sleep(Duration::from_secs(4));
            for x in joins {
                x.join();
            }
        }
    }

}
