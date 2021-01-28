use std::io;
use std::io::prelude::*;
use std::fs::File;

// create file and write something
fn create_file(filename: &str, buf: &[u8]) -> io::Result<()> {
    let mut f = File::create(filename).unwrap();
    f.write(&buf).unwrap();
    Ok(())
}

// read from file to String
fn read_file(filename: &str, buf: &mut String) -> io::Result<()> {
    let mut f = File::open(filename).unwrap();
    f.read_to_string(buf).unwrap();
    Ok(())
}

fn main() {
    let f = "foo.txt";
    let mut buf = String::new();
    match create_file(f, b"Hello, World!") {
        Ok(()) => {
            match read_file(f, &mut buf) {
                Ok(()) => { println!("{}", buf); }
                Err(e) => { println!("{}", e); }
            };
        }
        Err(e) => { println!("{}", e); }
    }

    use std::fs::OpenOptions;
    // 根据选项
    let file = OpenOptions::new().write(false).truncate(false).read(true).open("foo.txt").unwrap();
}
