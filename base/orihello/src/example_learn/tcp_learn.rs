#[test]
fn test_tcp() {
    use std::io::Read;
    use std::io::Write;
    use std::net::TcpStream;

    #[warn(dead_code)]
    fn handle_client(mut stream: TcpStream) {
        // read 20 bytes at a time from stream echoing back to stream
        loop {
            let mut read = [0; 1028];
            match stream.read(&mut read) {
                Ok(n) => {
                    if n == 0 {
                        // connection was closed
                        break;
                    }
                    stream.write(&read[0..n]).unwrap();
                }
                Err(err) => {
                    panic!(err);
                }
            }
        }
    }
    use std::net::TcpListener;
    use std::thread;
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(_) => {
                println!("Error");
            }
        }
    }
}