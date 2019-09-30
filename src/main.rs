const fn init_len()->usize {
    return 5;
}
fn main(){
    let arr = [0;init_len()];
    print!("{}",arr.len())
}