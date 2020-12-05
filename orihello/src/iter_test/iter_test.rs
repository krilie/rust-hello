pub fn test() {
    let v = vec![1, 2, 3, 23, 23];
    for x in v {
        println!("{:?}", x);
    }
}

pub fn test2() {
    let v = vec![1, 2, 3, 23, 12, 23];
    {
        //        let mut _iterator = v.into_iter(); // 所有权转移
        let mut _iterator = v.iter();
        loop {
            match _iterator.next() {
                Some(i) => println!("{:?}", i),
                None => break,
            }
        }
    }
    println!("{:?}", v);
}

pub fn test4() {
    let mut arr = [1, 23, 4, 3, 4, 23];
    for i in arr.iter_mut() {
        *i += *i;
    }
    println!("{:?}", arr);
}
