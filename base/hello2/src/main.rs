fn main() {
    println!("Hello, world!");
    say_hi("sss");
    say_what("what",hi);
    say_what("what",hello);
}

fn say_hi(name: &str){
    println!("Hi,{}",name);
}

fn hi(name :&str){
    println!("hi,{}",name);
}
fn hello(name:&str){
    println!("hello,{}",name);
}
fn say_what(name:&str,func: fn(&str)){
    func(name);
}
