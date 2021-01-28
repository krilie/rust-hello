use std::io::Write;

fn main() {
    use std::io;

    fn read_from_stdin(buf: &mut String) -> io::Result<()> {
        return match io::stdin().read_line(buf) {
            Ok(a) => {
                println!("{}", a);
                Ok(())
            }
            Err(e) => {
                println!("{}", e);
                io::Result::Err(e)
            }
        };
    }
    let mut line = "".to_string();
    read_from_stdin(&mut line).unwrap();
    println!("{}", line);
    // writer
    io::stdout().write(line.as_bytes());
}
