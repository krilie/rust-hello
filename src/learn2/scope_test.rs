pub fn test(){
    let v="hello world";
    assert_eq!(v,"hello world");
    println!("{}",v);
    let v = "hello rust";
    println!("{}",v);
    assert_eq!(v,"hello rust");
    {
        let v = "hello World";
        assert_eq!(v,"hello World");
        println!("{}",v);
    }
    assert_eq!(v,"hello rust");
    println!("{}",v);
}