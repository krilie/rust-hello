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

pub fn test2() {
    fn swap((x, y): (&str, i32)) -> (i32, &str) {
        (y, x)
    }
    let t = ("aa", 18);
    let t = swap(t);
    println!("{}-{}", t.0, t.1);
}

pub fn test3() {
    fn addsub(x: isize, y: isize) -> (isize, isize) {
        (x + y, x - y)
    }
    let (a, b) = addsub(5, 8);
    println!("{:?}", a);
    println!("{:?}", b);
}

pub fn test4() {
    fn gcd(a: u32, b: u32) -> u32 {
        if b == 0 {
            return a;
        }
        return gcd(b, a % b);
    }
    let g = gcd(60, 40);
    println!("{:?}", g);
}

pub fn test5(){
    use std::ops::Mul;
    fn square<T:Mul<T,Output=T>>(x:T,y:T) ->T{x*y}
    let a = square(37,41);
    let b = square(37.2,41.1);
    println!("{:?}", a);
    println!("{:?}", b);
}