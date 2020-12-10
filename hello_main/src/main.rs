fn main() {
    let xm = ("xiaoming", 54);
    let xh = ("xiaohong", 66);
    print_id(xm);
    print_id(xh);
    print_name(xm);
    print_age(xh);
    print_name(xm);
    print_age(xh);
}

fn print_id((name, age): (&str, i32)) {
    println!("I'm {},age {}.", name, age);
}

fn print_age((_, age): (&str, i32)) {
    println!("My age is  {}", age);
}

fn print_name((name,_): (&str, i32)) {
    println!("I am  {}", name);
}
