fn main() {
    let number = 42;
    match number {
        0 => println!("Origin"),
        1...3 => println!("All"),
        3 | 4 | 13 => println!("Bad Luck"),
        n @ 42 => println!("answer is {}", n),
        _ => println!("common"),
    }
}
