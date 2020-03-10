mod thread_1 {
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test() {
        thread::spawn(|| {
            for i in 1..10 {
                println!("spawn {}", i);
                thread::sleep(Duration::from_secs(1));
            }
        });
        for i in 1..50 {
            println!("for {}", i);
            thread::sleep(Duration::from_secs(1));
        }
    }
}