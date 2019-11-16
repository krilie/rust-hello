// 声明宏 macro_rules!
// 过程宏

macro_rules! unless {
    ($arg:expr,$branch:expr) => {
        if !$arg {
            $branch
        };
    };
}
fn cmp(a: i32, b: i32) {
    unless!(a > b, {
        println!("{}<{}", a, b);
    });
}
pub fn test() {
    let (a, b) = (1, 2);
    cmp(a, b);
}
