mod sound;
mod voice;
fn main() {
    println!("Hello, world!");
    crate::sound::instrument::show();
    let mut v = crate::sound::Vegetable::new("aaa");
    crate::voice::hello_voice();
    println!("{}",v.name)
}
