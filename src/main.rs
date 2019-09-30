fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    loop {
        match v.pop() {
            Some(x) => println!("{}", x),
            None => break,
        }
    }
    while let Some(x) = v.pop(){
        println!("{}",x)
    }
}
