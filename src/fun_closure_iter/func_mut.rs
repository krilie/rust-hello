fn modify(mut v: Vec<u32>) -> Vec<u32> {
    v.push(42);
    v
}
fn modify_mut(v: &mut [u32]) {
    v.reverse();
}
pub fn test() {
    let v = vec![1, 2, 3];
    let v = modify(v);
    println!("{:?}", v);
    let mut b = v;
    modify_mut(&mut b);
    println!("{:?}", b);
}

pub fn test2(){
    fn swap((x,y):(&str,i32)) -> (i32,&str) {
        (y,x)
    }
    let t = ("aa",18);
    let t = swap(t);
    println!("{}",t);
}