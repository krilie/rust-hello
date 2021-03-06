use std::thread;

pub fn test() {
    let mut v = vec![];
    for id in 0..5 {
        let child = thread::spawn(move || {
            println!("{:?}", id);
        });
        v.push(child);
    }
    println!("in main...");
    for child in v {
        let _result = child.join();
    }
    println!("...");
}

pub fn test2() {
    let mut v = vec![];
    for id in 0..5 {
        let thread_name = format!("child-{}", id);
        let size: usize = 3 * 1024;
        let _ = thread::Builder::new()
            .name(thread_name).stack_size(size);
        let child = thread::spawn(move || {
            println!("in child:{}", id);
            if id == 3 {
                println!("3");
            }
        });
        v.push(child);
    }
    println!("in main...");
    for child in v {
        child.join().unwrap();
    }
    println!("...");
}