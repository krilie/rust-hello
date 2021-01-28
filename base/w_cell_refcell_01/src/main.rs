use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

fn main() {
    {
        // Cell<T> 只能用于 T 实现了 Copy 的情况；
        use std::cell::Cell;
        let c = Cell::new(5);
        let five = c.get(); // clone
        println!("{}", five);
    }
    {
        use std::cell::Cell;
        let c = Cell::new(5);
        c.set(10);
        println!("{:?}", c);
    }
    {
        let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));
        shared_map.borrow_mut().insert("africa", 92388);
        shared_map.borrow_mut().insert("kyoto", 11837);
        shared_map.borrow_mut().insert("piccadilly", 11826);
        shared_map.borrow_mut().insert("marbles", 38);
        println!("{:?}", shared_map);
        println!("{:?}", shared_map.borrow());
    }
}
