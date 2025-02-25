use crate::{data::accounts_data::AccountsData, model::Account, service::transaction_action::TransactionAction};
use std::{sync::Arc, thread, time::{Duration, Instant}};

use chrono::Utc;
use rand::Rng;

pub struct TransferService{
    account_data: Arc<AccountsData>,
}

impl TransferService {
    pub fn new() -> Self{
        Self{
            account_data: AccountsData::new()
        }
    }

    // Pulls accounts in the apropriate order and starts transfer process.
    pub fn start_transfer(&self, from_account_id: i32, to_account_id: i32, customer: String, amount: i32) -> thread::JoinHandle<()> {
        let (from, to, action) = self.account_data.fetch_accounts(from_account_id, to_account_id);
        
        thread::spawn(move || TransferService::transfer(from, to, customer, amount, action))
    }

    // Acquire locks for both accounts and move funds between the two accounts.
    fn transfer(from: Arc<Account>, to: Arc<Account>, customer: String, amount: i32, action: TransactionAction) {
            let mut transaction_start = Instant::now();
            let mut retry_count = 0;
            println!("[{:?}]: Customer {} is starting transaction at {:?}", thread::current().id(), customer, Utc::now());
    
            // Generate random processing time.
            let mut rng = rand::rng();
            let processing_time: u64 = rng.random_range(100..=1000);
            
            // Deadlock detection
            while retry_count <= 3{
                // Lock from account and attempt to lock to account. Note the initial lock on from account is blocking.
                let from_balance_lock = from.balance.lock();
                let to_balance_lock = to.balance.try_lock();
    
                // If both locks are acquired perform transaction and return from function.
                if to_balance_lock.is_ok(){
                    println!("Thread acquired both account locks. Processing transaction.");
                    thread::sleep(Duration::from_millis(processing_time));

                    let mut to_balance = to_balance_lock.unwrap();
                    let mut from_balance = from_balance_lock.unwrap();
    
                    // Uses transaction action to determine which direction the funds should be transfered.
                    match action {
                        TransactionAction::Withdraw => {
                            if *to_balance - amount < 0 {
                                panic!("Not enough funds to process request!");
                            }

                            *from_balance += amount;
                            *to_balance -= amount;

                            println!("[{:?}]: Transfer {} from Account {:?} to Account {:?} completed at {:?}", thread::current().id(), amount, to.id, from.id, Utc::now());
                        }
                        TransactionAction::Deposit => {
                            if *from_balance - amount < 0 {
                                panic!("Not enough funds to process request!");
                            }

                            *from_balance -= amount;
                            *to_balance += amount;

                            println!("[{:?}]: Transfer {} from Account {:?} to Account {:?} completed at {:?}", thread::current().id(), amount, from.id, to.id, Utc::now());
                        }
                    }
                    return;
                }

                // Deadlock detection. Allows multiple deadlocks retries until second lock is acquired.
                if transaction_start.elapsed() > Duration::from_millis(1000) && retry_count < 3 {
                    // Drop lock on retry
                    drop(from_balance_lock);

                    // Performs a progressive delay of 1, 3, and 5 seconds for the retries.
                    let progressive_backoff = vec![1, 3, 5];
                    thread::sleep(Duration::from_secs(progressive_backoff[retry_count]));

                    retry_count += 1;
                    transaction_start = Instant::now();
                    println!("Locks for accounts {} and {} could not be acquired. Retrying transaction. Attempt number: {}", from.id, to.id, retry_count);
                }
            }
            // Final deadlock detection. Only occurs after all retries fail.
            println!("Locks could not be acquired. Deadlock occured cancelling transaction.");
    }

    pub fn print_final_accounts(&self) {
        for acc in self.account_data.get_account_list().values() {
            println!("{:?}", acc)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_transfer_amount() {
        let transfer_service = TransferService::new();

        let transfer_thread = transfer_service.start_transfer(1, 3, String::from("test"), 500);

        transfer_thread.join().unwrap();

        let transfer_accounts = transfer_service.account_data.get_account_list();
        
        assert_eq!(*transfer_accounts.get(&1).unwrap().balance.lock().unwrap(), 9500);
        assert_eq!(*transfer_accounts.get(&3).unwrap().balance.lock().unwrap(), 30500);
    }

    #[test]
    fn should_transfer_amount_in_order() {
        let transfer_service = TransferService::new();

        let transfer_thread = transfer_service.start_transfer(5, 1, String::from("test"), 500);

        transfer_thread.join().unwrap();

        let transfer_accounts = transfer_service.account_data.get_account_list();
        
        assert_eq!(*transfer_accounts.get(&1).unwrap().balance.lock().unwrap(), 10500);
        assert_eq!(*transfer_accounts.get(&5).unwrap().balance.lock().unwrap(), 19500);
    }

    #[test]
    #[should_panic]
    fn should_panic_when_overdraft() {
        let transfer_service = TransferService::new();

        let transfer_thread = transfer_service.start_transfer(1, 3, String::from("test"), 10500);

        transfer_thread.join().unwrap();
    }
}