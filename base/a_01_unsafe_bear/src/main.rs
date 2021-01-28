fn main() {
    let a = 1;
    let b = &a as *const i32;

    let mut x = 2;
    let y = &mut x as *mut i32;
    {
        let a =1;
        let b = &a as *const i32;
        let c = unsafe{*b};
        println!("{}", c);
    }
    {
        let a: Box<i32> = Box::new(10);
        // 我们需要先解引用a，再隐式把 & 转换成 *
        let b: *const i32 = &*a;
        // 使用 into_raw 方法
        let c: *const i32 = Box::into_raw(a);
        let d = unsafe{*c};
        println!("{}", d);
    }
    {
        // 显式
        let a = 1;
        let b: *const i32 = &a as *const i32; //或者let b = &a as *const i32；
        // 隐式
        let c: *const i32 = &a;
        unsafe {
            println!("{}", *c);
        }
    }
}
