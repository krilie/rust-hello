pub fn test(){
    let num = 42;
    match num {
        0=>println!("origin"),
        1..=3=>println!("all"),
        |5|7|13 => println!("back luck"),
        n@42 => println!("ok {}",n),
        _ =>println!("common"),
    }
}