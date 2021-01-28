use std::thread;
use std::sync::Arc;

fn main() {
    let var : Arc<i32> = Arc::new(5);
    let share_var = var.clone();

    // 创建一个新线程
    let new_thread = thread::spawn(move|| {
        println!("share value in new thread: {}, address: {:p}", share_var, &*share_var);
    });

    // 等待新建线程先执行
    new_thread.join().unwrap();
    println!("share value in main thread: {}, address: {:p}", var, &*var);
}
