fn foo<T>(x:T)->T{
    return x;
}
pub fn test(){
    println!("{:?}", foo(1));
    println!("{:?}", foo("hello"));
    println!("{:?}", foo(2.34234f64));
    println!("{:?}", foo(2.34234f32));
}