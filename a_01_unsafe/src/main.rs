fn main() {
    // 解引用一个裸指针*const T和*mut T
    let x = 5;
    let raw = &x as *const i32;
    let points_at = unsafe { *raw };
    println!("raw points at {}", points_at);
}

#[test]
fn read_mut() {
    static mut N: i32 = 5;
    unsafe {
        N += 1;
        println!("N: {}", N);
    }
}

#[test]
fn unsafe_func() {
    unsafe trait Scary { }
    unsafe impl Scary for i32 {}
    unsafe impl Scary for i64 {}

    static mut N: i32 = 5;
    unsafe fn foo() {
        N += 1;
        println!("N: {}", N);
    }
    unsafe {
        foo();
    }
}
