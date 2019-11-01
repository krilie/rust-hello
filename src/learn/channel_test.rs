use std::sync::mpsc::sync_channel;
use std::thread;

pub fn test(){
    let (tx,rx) = sync_channel(1);
    tx.send(1).unwrap();
    thread::spawn(move||{tx.send(2).unwrap()});
    println!("{}",rx.recv().unwrap());
    println!("{}",rx.recv().unwrap());
}