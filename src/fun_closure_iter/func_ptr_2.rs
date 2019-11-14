fn sum(a:i32,b:i32) ->i32 {
    a+b
}
fn product(a:i32,b:i32) ->i32 {
    a*b
}
type MathOp = fn(i32,i32)->i32;
fn math(op:&str,a:i32,b:i32)->i32{
    match op{
        "sum"=>sum(a,b),
        _=>product(a,b),
    }
}
pub fn test(){
    let a = 2;
    let b = 3;
    let sum = math("sum",a,b);
    println!("{}",sum);
}