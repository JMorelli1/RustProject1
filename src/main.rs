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
    // let account = Arc::new(Mutex::new(Account {balance: 1000}));
    // let mut transactions = vec![];

    // for i in 1..=10 {
    //     let account_clone = Arc::clone(&account);

    //     let transaction = thread::spawn(move || {
    //         println!("Thread {} is processing transaction", i);
    //         let mut account = account_clone.lock().unwrap();
    //         account.balance += 100;
    //         println!("Thread {} deposited $100, New balance is: ${}", i, account.balance);
    //     });

    //     transactions.push(transaction);
    // }

    // for transaction in transactions {
    //     transaction.join().unwrap();
    // }

    // println!("All transactions completed!");
    // let final_balance = account.lock().unwrap().balance;
    // println!("Final balance is {}", final_balance);
}