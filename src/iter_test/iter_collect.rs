pub fn test() {
    let a = [1, 2, 3];
    println!("{:?}", a.iter().any(|&x| x != 2));
    let sum = a.iter().fold(0, |acc, x| acc + *x);
    println!("{:?}", sum);
}

pub fn test2() {
    let arr = [1, 2, 3];
    let result1 = arr.iter().any(|&x| x != 2);
    let result2 = arr.iter().any(|x| *x != 2);
    println!("## {:?}", result1);
    println!("## {:?}", result2);
}

pub fn test3() {
    let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!(":{:?}", arr.iter().fold(0, |acc, x| acc + *x));
    println!(":{:?}", arr.iter().fold(0, |acc, &x| acc + x));
    println!(":{:?}", arr.into_iter().fold(0, |acc, x| acc + x));
}

pub fn test4(){
    let arr:Vec<i32> = vec![1i32,2i32,3i32];
    let arr2 = arr.iter().map(|x|*x as u32).collect::<Vec<u32>>();
    println!("{:?}", arr2);
}


