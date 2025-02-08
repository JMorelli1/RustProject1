use std::thread;
use std::sync::{Arc, Mutex};

struct Account {
    balance: i32,
}

fn main() {
    let account = Arc::new(Mutex::new(Account {balance: 1000}));
    let mut transactions = vec![];

    for i in 1..=10 {
        let account_clone = Arc::clone(&account);

        let transaction = thread::spawn(move || {
            println!("Thread {} is processing transaction", i);
            let mut account = account_clone.lock().unwrap();
            account.balance += 100;
            println!("Thread {} deposited $100, New balance is: ${}", i, account.balance);
        });

        transactions.push(transaction);
    }

    for transaction in transactions {
        transaction.join().unwrap();
    }

    println!("All transactions completed!");
    let final_balance = account.lock().unwrap().balance;
    println!("Final balance is {}", final_balance);
}