#[allow(dead_code)]
fn func_name(arg1: u32, arg2: String) -> Vec<u32> {
    vec![1, 2, 3]
}

#[allow(dead_code)]
fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

pub fn test(){
    println!("{:?}", r#match("foo", "foobar"));
}