use std::thread;

static mut VAR: i32 = 5;

fn main() {
    // // 创建一个新线程
    // let new_thread = thread::spawn(move|| {
    //     println!("static value in new thread: {}", VAR);
    // });
    //
    // // 等待新线程先运行
    // new_thread.join().unwrap();
    // println!("static value in main thread: {}", VAR);
}

#[test]
fn v001() {
    // 创建一个新线程
    let new_thread = thread::spawn(move|| {
        unsafe {
            println!("static value in new thread: {}", VAR);
            VAR = VAR + 1;
        }
    });

    // 等待新线程先运行
    new_thread.join().unwrap();
    unsafe {
        println!("static value in main thread: {}", VAR);
    }
}
