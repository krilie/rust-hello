fn main() {
    let out = 42;
//    fn add(i: i32, j: i32) -> i32 {
//        i + j + out
//    };
    let closure_annotated = |i: i32, j: i32| -> i32 { i + j + out };
    let closure_inferred = |i, j| i + j + out;
    let i = 1;
    let j = 2;
    print!("{}\n", closure_annotated(i, j));
    print!("{}\n", closure_inferred(i, j))
}
