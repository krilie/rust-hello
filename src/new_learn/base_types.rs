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

#[test]
fn test_never() {
    fn foo() -> i32 {
        let x: ! = {
            return 13;
        };
    }
    println!("{}", foo());
}

#[test]
fn tuple() {
    fn move_coords(x: (i32, i32)) -> (i32, i32) {
        return (x.0 + 1, x.1 + 1);
    }
    println!("{:?}", move_coords((23, 32)));
}

#[test]
fn tuple_ls() {
    let tuple: (&'static str, i32, char) = ("hello", 5, 'c');
    println!("{}", tuple.0);
    println!("{}", tuple.1);
    println!("{}", tuple.2);
    #[derive(Debug)]
    struct Color(i32, i32, i32, i32);
    let color = Color(1, 2, 3, 4);
    println!("{:?}", color);
}

#[test]
fn enum_test() {
    enum Number {
        Zero,
        One,
        Two,
    }
    let a = Number::Zero;
    match a {
        Number::Zero => println!("0"),
        Number::One => println!("1"),
        Number::Two => println!("2"),
    }
}

#[test]
fn vec_test(){
    let mut v1 = vec![1,2,3,4,5];
    println!("{:?}", v1.pop());
}
#[test]
fn box_test(){
    #[derive(PartialEq,Debug)]
    struct TT(i32,i32);
    let bb = Box::new(TT(32,32));
    let pbb = *bb;
//    println!("{:?}", bb);
    println!("{}", pbb.0);
}