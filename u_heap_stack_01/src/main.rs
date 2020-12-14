// fn main() {
//     let b = foo("world");
//     println!("{}", b);
// }
//
// fn foo(x: &str) -> &str {
//     let a = "Hello, ".to_string() + x;
//     &a
// }

fn main() {
    let b = foo("world");
    println!("{}", b);
}

fn foo(x: &str) -> String {
    let a = "Hello, ".to_string() + x;
    a
}
