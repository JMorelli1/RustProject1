use std::result;
use std::time::Instant;
use std::{thread, time::Duration};
use std::sync::{Arc, Mutex, MutexGuard};
use rand::Rng;

// Simple account object to handle balance data. Derive annotation allows for {:?} logging object.
#[derive(Debug)]
struct Account {
    balance: i32,
}

// fn transfer(from: &Arc<Mutex<Account>>, to: &Arc<Mutex<Account>>, amount: i32, processing_time: u64){
//     let transaction_start = Instant::now();
//     let mut from_lock = from.lock().unwrap();
//     println!("{:?} locked", &*from_lock);
//     // thread::sleep(Duration::from_millis(1000));

//     while transaction_start.elapsed() < Duration::from_millis(200) {
//         let to_lock = to.try_lock();

//         if let Ok(_guard) = &to_lock {
//             println!("Both locks acquired.");

//             from_lock.balance -= amount;
//             to_lock.unwrap().balance += amount;

//             println!("Transferring {} from {:?} to {:?}", amount, from_lock, "test");
//             return;
//         }
//     }
    
//     println!("Thread could not acquire both locks. Deadlock occured, cancelling transaction.")
// }

fn transfer(from: Arc<Mutex<Account>>, to: Arc<Mutex<Account>>, thread_id: i32) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let transaction_start = Instant::now();
        println!("Thread {} is starting transaction", thread_id);

        // Generate random processing time and deposit amount.
        let mut rng = rand::rng();
        let processing_time: u64 = rng.random_range(100..=1000);
        let deposit: i32 = rng.random_range(50..=1000);

        while transaction_start.elapsed() < Duration::from_millis(500){
            let from_lock = from.try_lock();

            if from_lock.is_ok() {
                let to_lock = to.try_lock();

                if to_lock.is_ok(){
                    println!("Thread {} acquired both account locks. Processing transaction.", thread_id);

                    from_lock.unwrap().balance -= deposit;
                    to_lock.unwrap().balance += deposit;

                    println!("Transferring {} from {:?} to {:?}", deposit, &from.lock(), &to.lock());
                    return;
                }
            }
        }
        println!("Lock could not be acquired. Deadlock occured.");

        // let mut from_lock = from.try_lock();
        // let transaction_result = to.try_lock().map(| mut to_lock| {
        // //    from_lock.balance -= deposit;
        //    to_lock.balance += deposit; 

        // //    println!("Transferring {} from {:?} to {:?}", deposit, from_lock, to_lock);
        // println!("Transferring {} to {:?}", deposit, to_lock);
        // });

        // if transaction_result.is_err() {
        //     println!("Could not acquire both locks. Transaction cancelled.")
        // }

        // transfer(&from, &to, deposit, processing_time);

        // println!("Thread {} finished its transaction in {} milliseconds, Transaction was succesfull: {}", thread_id, processing_time, transaction_result.is_ok());
    })
}


fn main() {
    // Create new Accounts mutex.    
    let account1 = Arc::new(Mutex::new(Account { balance : 20000 }));
    let account2 = Arc::new(Mutex::new(Account { balance : 50000 }));
    let mut transactions = vec![];

    
    for thread_id in 1..=10 {
        // Create 10 new threads
        let transaction1 = transfer(Arc::clone(&account1), Arc::clone(&account2), thread_id);
        let transaction2 = transfer(Arc::clone(&account2), Arc::clone(&account1), thread_id+10);

        transactions.push(transaction1);
        transactions.push(transaction2);
    }

    // Wait for all threads to finish.
    for transaction in transactions {
        transaction.join().unwrap();
    }

    println!("All transactions finished!");
    println!("Final balance of accounts is {} {}", account1.lock().unwrap().balance, account2.lock().unwrap().balance);
}