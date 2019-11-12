use std::fmt::Debug;

fn match_option<T: Debug>(o: Option<T>) {
    match o {
        Some(i) => println!("{:?}", i),
        None => println!("nothing"),
    }
}

pub fn test() {
    let a = Some(3);
    match_option(a);
    match_option(Some("some"));
    let b: Option<i8> = None;
    match_option(b);
}
