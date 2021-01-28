fn main() {}


#[test]
fn option_test() {
    fn guess(n: i32) -> bool {
        if n < 1 || n > 10 {
            panic!("Invalid number: {}", n);
        }
        n == 5
    }
    println!("{}", guess(11));
}

#[test]
fn option_test2() {
    fn find(haystack: &str, needle: char) -> Option<usize> {
        for (offset, c) in haystack.char_indices() {
            if c == needle {
                return Some(offset);
            }
        }
        None
    }
    println!("{:?}", find("aaa", 'a'));
    match find("bbb", 'a') {
        None => println!("none"),
        Some(i) => println!("some {}", i),
    }
    let b = find("accc", 'c').unwrap();
    println!("{}", b);

    fn extension_explicit(file_name: &str) -> Option<&str> {
        match find(file_name, '.') {
            None => None,
            Some(i) => Some(&file_name[i + 1..]),
        }
    }
    match extension_explicit("foo.rs") {
        None => println!("no extension"),
        Some(ext) => assert_eq!(ext, "rs"),
    }
}

#[test]
fn get_file_name() {
    fn double_number(number_str: &str) -> i32 {
        2 * number_str.parse::<i32>().unwrap()
    }

    let n: i32 = double_number("10");
    assert_eq!(n, 20);
}
