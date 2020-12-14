macro_rules! m1 { () => (()) }

// 宏 m1 在这里可用

mod foo {
    // 宏 m1 在这里可用

    #[macro_export]
    macro_rules! m2 { () => (()) }

    // 宏 m1 和 m2 在这里可用
}

// 宏 m1 在这里可用
#[macro_export]
macro_rules! m3 { () => (()) }

// 宏 m1 和 m3 在这里可用

// #[macro_use(m3)]
// extern crate foo;
// // foo 中只有 m3 被导入
#[macro_use]
mod bar {
    // 宏 m1 和 m3 在这里可用

    macro_rules! m4 { () => (()) }

    // 宏 m1, m3, m4 在这里均可用
}

// 宏 m1, m3, m4 均可用
fn main() {
    println!("hello")
}

// #[macro_use]
// extern crate foo;
// // foo 中 m2, m3 都被导入
