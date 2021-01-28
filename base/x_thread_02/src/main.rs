use std::thread;

fn main() {
    // 创建一个线程，线程名称为 thread1, 堆栈大小为4k
    let new_thread_result = thread::Builder::new()
        .name("thread1".to_string())
        .stack_size(4*1024*1024).spawn(move || {
        println!("I am thread1.");
    });
    // 等待新创建的线程执行完成
    new_thread_result.unwrap().join().unwrap();

        // 创建一个线程
        let new_thread = thread::spawn(move || {
            // 再创建一个线程
            thread::spawn(move || {
                loop {
                    println!("I am a new thread.");
                }
            })
        });

        // 等待新创建的线程执行完成
        new_thread.join().unwrap();
        println!("Child thread is finish!");

        // 睡眠一段时间，看子线程创建的子线程是否还在运行
        thread::sleep_ms(100);


}
