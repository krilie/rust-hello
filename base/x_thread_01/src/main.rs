use std::thread;

fn main() {
    // 创建一个线程
    let new_thread = thread::spawn(move || {
        println!("I am a new thread.");
    });
    // 等待新建线程执行完成
    new_thread.join().unwrap();
}
