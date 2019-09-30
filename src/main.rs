fn closure_math<F: Fn() -> i32>(op: F) -> i32 {
    op()
}

fn main() {
    let a = 2;
    let b = 3;
    println!("{}", closure_math(|| a + b));
    println!("{}", closure_math(|| a * b));
}
