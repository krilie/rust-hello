pub fn temp()->i32{
    return 1;
}
pub fn test(){
    let x=&temp();
    temp() = *x;
}