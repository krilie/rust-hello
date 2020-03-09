use std::fmt::Debug;

trait DoSomething<T> {
    fn do_sth(&self, value: T);
}

impl<'a, T: Debug> DoSomething<T> for &'a usize {
    fn do_sth(&self, value: T) {
        println!("{:?}", value);
    }
}
//fn foo<'a>(b: Box<dyn DoSomething<&'a usize>>) {
//    let s: usize = 10;
//    b.do_sth(&s);
//}

// 高阶生命周期函数
fn bar(b: Box<dyn for<'f> DoSomething<&'f usize>>) {
    let s: usize = 10;
    b.do_sth(&s);
}

pub fn test() {
    let x = Box::new(&2usize);
    //    foo(x.clone());
    bar(x);
}
