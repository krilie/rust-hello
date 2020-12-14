macro_rules! create_function {
 ($func_name:ident)=>(
    fn $func_name(){
      println!("function {:?} is called",stringify!($func_name))
    }
 )
}

fn main() {
    println!("Hello, world!");
    create_function!(foo);
    foo();
}
