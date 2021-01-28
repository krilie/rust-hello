/*
ident: 标识符，用来表示函数或变量名
expr: 表达式
block: 代码块，用花括号包起来的多个语句
pat: 模式，普通模式匹配（非宏本身的模式）中的模式，例如 Some(t), (3, 'a', _)
path: 路径，注意这里不是操作系统中的文件路径，而是用双冒号分隔的限定名(qualified name)，如 std::cmp::PartialOrd
tt: 单个语法树
ty: 类型，语义层面的类型，如 i32, char
item: 条目，
meta: 元条目
stmt: 单条语句，如 let a = 42;
*/
macro_rules! bar {
    ($x:ident) => { println!("The argument you passed to macro is {}", $x); }
}

fn main() {
    let a = 42;
    foo(a);
    bar!(a);
}

fn foo(x: i32) {
    println!("The argument you passed to function is {}", x);
}

