use std::time::Instant;
use std::{thread, time::Duration};
use std::sync::{Arc, Mutex};
use rand::Rng;

// Simple account object to handle balance data. Derive annotation allows for {:?} logging object.
#[derive(Debug)]
struct Account {
    balance: i32,
}

enum TransactionAction{
    Deposit,
    Withdraw
}

fn transfer(from: Arc<Mutex<Account>>, to: Arc<Mutex<Account>>, thread_id: i32, action: TransactionAction) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let transaction_start = Instant::now();
        println!("Thread {} is starting transaction", thread_id);

        // Generate random processing time and deposit amount.
        let mut rng = rand::rng();
        let processing_time: u64 = rng.random_range(100..=1000);
        let deposit: i32 = rng.random_range(50..=1000);

        // Deadlock detection
        while transaction_start.elapsed() < Duration::from_millis(1000){
            // Lock from account and attempt to lock to account. Note the initial lock on from is blocking.
            let from_lock = from.lock();
            let to_lock = to.try_lock();

            // If both locks are acquired perform transaction and return from function.
            if to_lock.is_ok(){
                println!("Thread {} acquired both account locks. Processing transaction.", thread_id);
                thread::sleep(Duration::from_millis(processing_time));

                match action {
                    TransactionAction::Withdraw => {
                        from_lock.unwrap().balance += deposit;
                        to_lock.unwrap().balance -= deposit;
                    }
                    TransactionAction::Deposit => {
                        from_lock.unwrap().balance -= deposit;
                        to_lock.unwrap().balance += deposit;
                    }
                }

                println!("Transferring {} from {:?} to {:?}", deposit, &from.lock(), &to.lock());
                return;
            }
        }
        // Deadlock detection.
        println!("Locks in Thread {} could not be acquired. Deadlock occured cancelling transaction.", thread_id);
    })
}


fn main() {
    // Create new Accounts mutex.    
    let account1 = Arc::new(Mutex::new(Account { balance : 20000 }));
    let account2 = Arc::new(Mutex::new(Account { balance : 50000 }));
    let mut transactions = vec![];

    
    for thread_id in 1..=10 {
        // Start 10 new threads with PROPER account access ordering. This will cause deadlocks.
        let transaction1 = transfer(Arc::clone(&account1), Arc::clone(&account2), thread_id, TransactionAction::Deposit);
        let transaction2 = transfer(Arc::clone(&account1), Arc::clone(&account2), thread_id+10, TransactionAction::Withdraw);

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