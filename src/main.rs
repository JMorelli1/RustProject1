mod model;
use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use crate::model::Account;

fn transfer(from: &Arc<Mutex<Account>>, to: &Arc<Mutex<Account>>, amount: i32){
    let mut from_lock = from.lock().unwrap();
    println!("{:?} locked", &*from_lock);
    thread::sleep(Duration::from_millis(1000));
    let mut to_lock = to.lock().unwrap();

    from_lock.balance -= amount;
    to_lock.balance += amount;

    println!("Transferring {} from {:?} to {:?}", amount, &*from_lock, &*to_lock);
}

fn create_deadlock(){
    let account1 = Arc::new(Mutex::new(Account { balance : 2000 }));
    let account2 = Arc::new(Mutex::new(Account { balance : 5000 }));

    let a1 = Arc::clone(&account1);
    let a2 = Arc::clone(&account2);

    let a1_transaction = thread::spawn(move || {
        transfer(&a1, &a2, 100);
    });

    let a1_clone = Arc::clone(&account1);
    let a2_clone = Arc::clone(&account2);

    let a2_transaction = thread::spawn(move || {
        transfer(&a2_clone, &a1_clone, 50);
    });

    a1_transaction.join().unwrap();
    a2_transaction.join().unwrap();
}

fn main() {
    create_deadlock();
}

// use std::{fs, os::unix::fs::FileTypeExt, process::Command, thread, time::Duration};

// const FIFO_PATH: &str = "/tmp/project1_fifo";

// fn create_fifo(){
    
//     if !fs::metadata(FIFO_PATH).map(|m| m.file_type().is_fifo()).unwrap_or(false){
//         Command::new("mkfifo")
//         .arg(FIFO_PATH)
//         .status()
//         .expect("Failed to create FIFO pipe");
//     }
// }

// fn main() {
//     let num_producers = 3;
//     create_fifo();

//     for i in 0..num_producers {
//         thread::spawn(move || {
//             let producer_name = format!("Producer-{}", i + 1);
//             let mut producer = Command::new("cargo")
//             .arg("run")
//             .arg("--bin")
//             .arg("producer")
//             .arg("--")
//             .arg(producer_name)
//             .spawn()
//             .expect("Failed to start producer");

//             producer.wait().expect("Producer process failed");
//         });
//     }

//     thread::sleep(Duration::from_millis(2000));

//     let consumer_thread = thread::spawn(move || {
//         let mut consumer = Command::new("cargo")
//         .arg("run")
//         .arg("--bin")
//         .arg("consumer")
//         .spawn()
//         .expect("Failed to start consumer");
        
//         consumer.wait().expect("Consumer process failed");
//     });

//     consumer_thread.join().unwrap();
// }