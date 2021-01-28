fn main() {
    println!("Hello, world!");
}

#[test]
fn one() {
    use std::sync::{Arc, Mutex, Condvar};
    use std::thread;

    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    // 创建一个新线程
    thread::spawn(move|| {
        let &(ref lock, ref cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
        println!("notify main thread");
    });

    // 等待新线程先运行
    let &(ref lock, ref cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        println!("before wait");
        started = cvar.wait(started).unwrap();
        println!("after wait");
    }
}

// 原子类型
#[test]
fn atomic_test() {
    use std::thread;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    let var : Arc<AtomicUsize> = Arc::new(AtomicUsize::new(5));
    let share_var = var.clone();

    // 创建一个新线程
    let new_thread = thread::spawn(move|| {
        println!("share value in new thread: {}", share_var.load(Ordering::SeqCst));
        // 修改值
        share_var.store(9, Ordering::SeqCst);
    });

    // 等待新建线程先执行
    new_thread.join().unwrap();
    println!("share value in main thread: {}", var.load(Ordering::SeqCst));
}

#[test]
fn atomic_two() {
    use std::thread;
    use std::sync::{Arc, Mutex};

    let var : Arc<Mutex<u32>> = Arc::new(Mutex::new(5));
    let share_var = var.clone();

    // 创建一个新线程
    let new_thread = thread::spawn(move|| {
        let mut val = share_var.lock().unwrap();
        println!("share value in new thread: {}", *val);
        // 修改值
        *val = 9;
    });

    // 等待新建线程先执行
    new_thread.join().unwrap();
    println!("share value in main thread: {}", *(var.lock().unwrap()));
}

#[test]
fn thread_one() {
    use std::sync::{Arc, Mutex, Condvar};
    use std::thread;

    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    // 创建一个新线程
    thread::spawn(move|| {
        let &(ref lock, ref cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
        println!("notify main thread");
    });

    // 等待新线程先运行
    let &(ref lock, ref cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        println!("before wait");
        started = cvar.wait(started).unwrap();
        println!("after wait");
    }
}


#[test]
fn prelude_do() {
    extern crate rayon;
    use rayon::prelude::*;

    let mut colors = [-20.0f32, 0.0, 20.0, 40.0,
        80.0, 100.0, 150.0, 180.0, 200.0, 250.0, 300.0];
    println!("original:    {:?}", &colors);

    colors.par_iter_mut().for_each(|color| {
        let c : f32 = if *color < 0.0 {
            0.0
        } else if *color > 255.0 {
            255.0
        } else {
            *color
        };
        *color = c / 255.0;
    });
    println!("transformed: {:?}", &colors);
}
