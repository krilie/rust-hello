pub fn test() {
    let a = vec![1, 2, 3];
    let mut iter = a.into_iter().map(|x| 2 * x);
    println!("{:?}", iter.next());
}

pub fn test2() {
    let arr1 = [1, 23, 4, 3, 56, 43];
    let c1 = arr1.iter().map(|x| 2 * x).collect::<Vec<i32>>();
    println!("{:?}", c1);
    let arr2 = ["1", "21", "23"];
    let c2 = arr2
        .iter()
        .filter_map(|x| x.parse().ok())
        .collect::<Vec<i32>>();
    println!("{:?}", c2);
    let arr3 = ['a', 'b', 'c'];
    for (idx, val) in arr3.iter().enumerate() {
        println!("idx:{:?},val:{:?}", idx, val);
    }
}
