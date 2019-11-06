pub fn test() {
    for n in 1..=101 {
        println!("{}", n);
    }
    let mut i = 0;
    while i < 100000 {
        println!("aabb {}", i);
        i += 1;
    }
}
