use std::thread;

pub fn test(){
    let mut v = vec![];
    for id in 0..5{
        let child = thread::spawn(move||{
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