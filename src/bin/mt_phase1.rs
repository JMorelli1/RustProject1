use std::{thread, time::Duration};
use rand::Rng;

/*
    Phase 1: Basic Thread Operations
    This program demonstrates basic thread creation and concurrency. It will create 10 threads, log their ids on start, and
    pause for a random number of milliseconds (between 100-500).
*/
fn main() {
    // Array for storing started threads
    let mut transactions = vec![];
    

    // Create 10 new threads and push them into array. Pauses for a random number of milli seconds to imitate "processing" time.
    for i in 1..=10 {
        let transaction = thread::spawn(move || {
            println!("Thread {} is processing transaction", i);

            let mut rng = rand::rng();
            let random_pause_time: u64 = rng.random_range(100..=500);

            thread::sleep(Duration::from_millis(random_pause_time));
            println!("Thread {} has completed it's transaction after {} milliseconds", i, random_pause_time);
        });
        
        transactions.push(transaction);
    }

    // Wait for all threads to finish.
    for transaction in transactions {
        transaction.join().unwrap();
    }

    println!("All transactions finished!");
}