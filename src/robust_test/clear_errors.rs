fn sum(a: i32, b: i32) -> i32 {
    a + b
}
pub fn test() {
    //    sum(1u32,2u32);
    println!("{:?}", sum(23, 23));
}

#[test]
fn it_works(){
    assert_eq!(2+2,4);
    println!("hh");
}

pub mod test2{
    #[allow(dead_code)]
    use linked_list;
    #[test]
    fn test(){

    }
}