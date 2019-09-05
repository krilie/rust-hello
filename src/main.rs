mod sound;
fn main() {
    println!("Hello, world!");
    crate::sound::instrument::show();
    let mut v = crate::sound::Vegetable::new("aaa");
    println!("{}",v.name)
}
