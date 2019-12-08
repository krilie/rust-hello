#[test]
fn number_test() {
    println!("{:?}", std::f32::INFINITY);
    println!("{:?}", std::f32::NEG_INFINITY);
    println!("{:?}", std::f32::NAN);
    println!("{:?}", std::f32::MIN);
    println!("{:?}", std::f32::MAX);
}

#[test]
fn test_slice() {
    let vec = vec![1, 2, 3, 4, 5];
    println!("{:?}", vec);
    println!("{:?}", &vec[1..]);
}

#[test]
fn test_str() {
    let truth: &'static str = "Rust 是一门优雅的语言";
    let ptr = truth.as_ptr();
    let len = truth.len();
    println!("len {}", len);
    let s = unsafe {
        let slice = std::slice::from_raw_parts(ptr, len);
        std::str::from_utf8(slice)
    };
    println!("s- {:?}\ntruth- {:?}", s, truth);
}

#[test]
fn raw_ptr_test() {
    let mut x = 10;
    let ptr_x = &mut x as *mut i32;
    let y = Box::new(20);
    let ptr_y = &*y as *const i32;
    unsafe {
        *ptr_x += *ptr_y;
    }
    println!("x{} y{}", x, y);
}