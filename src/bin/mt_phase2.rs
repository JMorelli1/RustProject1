use std::{thread, time::Duration};
use std::sync::{Arc, Mutex};
use rand::Rng;

// Simple account object to handle balance data.
struct Account {
    balance: i32,
}

/*
    Phase 2: Resource Protection
    This program demonstrates back mutex locks on shared resources. It will create 10 threads 
    which will attempt to deposit a random amount within a range of (100-500) milliseconds. Using locks
    the threads should all be able to act on the account balance without interferring with each other.
*/
fn main() {
    // Create new Account mutex.
    let account = Arc::new(Mutex::new(Account {balance: 1000}));
    let mut transactions = vec![];

    
    for i in 1..=10 {
        // Clone account so it can be "borrowed" by the new thread.
        let account_clone = Arc::clone(&account);

        // Create 10 new threads
        let transaction = thread::spawn(move || {
            println!("Thread {} is processing transaction", i);

            // Generate random processing time and deposit amount.
            let mut rng = rand::rng();
            let random_pause_time: u64 = rng.random_range(100..=500);
            let random_deposit: i32 = rng.random_range(50..=1000);

            // Acquire account lock.
            let mut account_lock = account_clone.lock().unwrap();
            
            // "Process" transaction and make deposit.
            thread::sleep(Duration::from_millis(random_pause_time));
            account_lock.balance += random_deposit;

            println!("Thread {} deposited ${}, New balance is: ${}", i, random_deposit, account_lock.balance);
            println!("Thread {} has completed it's transaction after {} milliseconds", i, random_pause_time);
        });

        transactions.push(transaction);
    }

    // Wait for all threads to finish.
    for transaction in transactions {
        transaction.join().unwrap();
    }

    println!("All transactions finished!");
    let final_balance = account.lock().unwrap().balance;
    println!("Final balance is {}", final_balance);
}