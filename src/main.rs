fn main() {
    let number = 3;
    match number {
        0 => println!("Origin"),
        1..=3 => println!("All"),
        n @ 3 | n @ 4 | n @ 13 => println!("Bad Luck {}", n),
        n @ 42 => println!("answer is {}", n),
        _ => println!("common"),
    }
}
