// #![feature(box_syntax, box_patterns)]
// fn main() {
//     let boxed = Some(box 5);
//     match boxed {
//         Some(box unboxed) => println!("Some {}", unboxed),
//         None => println!("None"),
//     }
// }

fn main() {
    let boxed = Some(Box::new(5));
    match boxed {
        Some(ref a) => println!("Some {}", a),
        None => println!("None"),
    }
}
