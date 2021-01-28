#[test]
pub fn test() {
    let d = [1, 2, 3, 4];
    match linear_search(&d, &6) {
        Some(a) => { println!("{:?}", a); }
        None => { println!("no some"); }
    }
}

#[allow(dead_code)]
pub fn linear_search<T>(arr: &[T], target: &T) -> Option<usize>
    where T: PartialEq
{
    for (index, item) in arr.iter().enumerate() {
        if item == target {
            return Some(index);
        }
    }
    Option::None
}

#[allow(dead_code)]
pub fn linear_search2<T>(arr: &[T], obj: &T) -> Option<usize>
    where T: PartialEq
{
    arr.iter().position(|x| x == obj)
}