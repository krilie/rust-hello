fn bubble_sort(a: &mut Vec<i32>) {
    let mut n = a.len();
    while n > 0 {
        let (mut i, mut max_ptr) = (1, 0);
        while i < n {
            if a[i - 1] > a[i] {
                a.swap(i - 1, i);
                max_ptr = i;
            }
            i += i;
        }
        n = max_ptr;
    }
}

pub fn test() {
    let mut a = vec![1, 4, 2, 3, 4, 3, 2, 8];
    bubble_sort(&mut a);
    println!("{:?}", a);
}
