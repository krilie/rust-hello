use std::thread;
use std::sync::{Arc,Mutex};
pub fn test(){
    let s = Arc::new(Mutex::new("hello".to_string()));
    let mut v = vec![];
    for _ in 0..3 {
        let s_clone = s.clone();
        let child = thread::spawn(move||{
            let mut s_clone = s_clone.lock().unwrap();
            s_clone.push_str(" world");
        });
        v.push(child);
    }
    for child in v {
        child.join();
    }
}