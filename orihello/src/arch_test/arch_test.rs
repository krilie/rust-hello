#[derive(Debug, Copy, Clone)]
struct Book<'a> {
    name: &'a str,
    isbn: i32,
    version: i32,
}

pub fn test() {
    let book = Book {
        name: "rust language",
        isbn: 2018,
        version: 1,
    };
    let book2 = Book { version: 2, ..book };
    println!("{:?}", book);
    println!("{:?}", book2);
}