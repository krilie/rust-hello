use std::net::{TcpListener, ToSocketAddrs, TcpStream};
use std::io::{Read, Write};
use std::{io, thread, env};

fn main() {
    println!("Hello, world!");
}

#[test]
fn greps() {
    use std::process::*;
    use std::env::args;

    let mut arg_iter = args();
    // panic if there is no one
    arg_iter.next().unwrap();
    let pattern = arg_iter.next().unwrap_or("main".to_string());
    let pt =  arg_iter.next().unwrap_or("./".to_string());
    let output = Command::new("/usr/bin/grep")
        .arg("-n")
        .arg("-r")
        .arg(&pattern)
        .arg(&pt)
        .output()
        .unwrap_or_else(|e| panic!("wg panic because:{}", e));
    println!("output:");
    let st = String::from_utf8_lossy(&output.stdout);
    let lines = st.split("\n");
    for line in lines {
        println!("{}", line);
    }
}

#[test]
fn two() {
    fn server<A: ToSocketAddrs>(addr: A) -> io::Result<()> {
        // 建立一个监听程序
        let listener = try!(TcpListener::bind(&addr)) ;
        // 这个程序一次只需处理一个链接就好
        for stream in listener.incoming() {
            // 通过match再次解包 stream到
            match stream {
                // 这里匹配的重点是如何将一个mut的匹配传给一个Result
                Ok(mut st) => {
                    // 我们总是要求client端先发送数据
                    // 准备一个超大的缓冲区
                    // 当然了，在实际的生活中我们一般会采用环形缓冲来重复利用内存。
                    // 这里仅作演示，是一种很低效的做法
                    let mut buf: Vec<u8> = vec![0u8; 1024];
                    // 通过try!方法来解包
                    // try!方法的重点是需要有特定的Error类型与之配合
                    let rcount = try!(st.read(&mut buf));
                    // 只输出缓冲区里读取到的内容
                    println!("{:?}", &buf[0..rcount]);
                    // 回写内容
                    let wcount = try!(st.write(&buf[0..rcount]));
                    // 以下代码实际上算是逻辑处理
                    // 并非标准库的一部分了
                    if rcount != wcount {
                        panic!("Not Fully Echo!, r={}, w={}", rcount, wcount);
                    }
                    // 清除掉已经读到的内容
                    buf.clear();
                }
                Err(e) => {
                    panic!("{}", e);
                }
            }
        }
        // 关闭掉Serve端的链接
        drop(listener);
        Ok(())
    }

    fn client<A: ToSocketAddrs>(addr: A) -> io::Result<()> {

        let mut buf = vec![0u8;1024];
        loop {
            // 对比Listener，TcpStream就简单很多了
            // 本次模拟的是tcp短链接的过程，可以看作是一个典型的HTTP交互的基础IO模拟
            // 当然，这个通讯里面并没有HTTP协议 XD！
            let mut stream = TcpStream::connect(&addr).unwrap();
            let msg = "WaySLOG comming!".as_bytes();
            // 避免发送数据太快而刷屏
            thread::sleep_ms(100);
            let rcount = try!(stream.write(&msg));
            let _ = try!(stream.read(&mut buf));
            println!("{:?}", &buf[0..rcount]);
            buf.clear();
        }
        Ok(())
    }

    let mut args = env::args();
    args.next();
    let action = args.next().unwrap();
    if action == "s" {
        server(&args.next().unwrap()).unwrap();
    } else {
        client(&args.next().unwrap()).unwrap();
    }

}
