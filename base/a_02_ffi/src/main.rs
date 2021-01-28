extern crate libc;

use libc::c_int;
use libc::c_void;
use libc::size_t;

#[link(name = "yourlib")]
extern {
    fn your_func(arg1: c_int, arg2: *mut c_void) -> size_t; // 声明ffi函数
    fn your_func2(arg1: c_int, arg2: *mut c_void) -> size_t;
    static ffi_global: c_int; // 声明ffi全局变量
}

fn main() {
    let result: size_t = unsafe {
        your_func(1 as c_int, Box::into_raw(Box::new(3)) as *mut c_void)
    };
}
