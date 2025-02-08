use std::{thread, time::Duration};

fn main() {
    let mut transactions = vec![];

    for i in 1..=10 {
        let transaction = thread::spawn(|| {
            println!("Thread has started");
        });
        transactions.push(transaction);
    }

    for transaction in transactions {
        println!("Thread {:?} is closing", transaction.thread().id());
        thread::sleep(Duration::from_millis(100));
        transaction.join().unwrap();
    }

    println!("All threads complete.");
}