/*
卫生宏最开始是由 Scheme 语言引入的，
后来好多语言基本都采用卫生宏，
即编译器或运行时会保证宏里面定义的变量或函数不会与外面的冲突，
在宏里面以普通方式定义的变量作用域不会跑到宏外面。
*/

macro_rules! foo {
    () => (let x = 3;);
}

macro_rules! bar {
    ($v:ident) => (let $v = 3;);
}

fn main() {
    // foo!();
    // println!("{}", x);
    bar!(a);
    println!("{}", a);
}

mod build {
    macro_rules! foo {
            () => (fn x() { println!("hello world") });
        }
    #[test]
    fn main() {
        foo!();
        x();
    }
}
