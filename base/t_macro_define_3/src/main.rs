macro_rules! vector {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(temp_vec.push($x);
            temp_vec.push($x);)*
            temp_vec
        }
    };
}

fn main() {
    let a = vector![1, 2, 4, 8];
    println!("{:?}", a);
}
