use std::ops::Drop;
#[derive(Debug)]
struct S(i32);
impl Drop for S {
    fn drop(&mut self) {
       println!("drop {}",self.0);
    }
}
pub fn test(){
    {
        #[allow(unused_variables)]
        let s = S(1);
    }
    #[allow(unused_variables)]
    let s = S(2);
}