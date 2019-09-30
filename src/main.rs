fn main() {
    let v = "hello world!";
    assert_eq!(v,"hello world!");
    let v = "hello rust!";
    assert_eq!(v,"hello rust!");
    {
        let v = "hello world!";
        assert_eq!(v,"hello world!");
    }
    assert_eq!(v,"hello rust!");
}