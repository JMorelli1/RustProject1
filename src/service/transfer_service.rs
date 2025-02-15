use crate::{data::accounts_data::AccountsData, service::transaction_action::TransactionAction};
use std::{sync::Arc, thread, time::{Duration, Instant}};

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

    pub fn transfer(&self, from_account_id: i32, to_account_id: i32, customer: String, amount: i32) -> thread::JoinHandle<()> {
        let (from, to, action) = self.account_data.fetch_accounts(from_account_id, to_account_id);
        thread::spawn(move || {
            let mut transaction_start = Instant::now();
            let mut retry_count = 0;
            println!("Customer {} is starting transaction", customer);
    
            // Generate random processing time.
            let mut rng = rand::rng();
            let processing_time: u64 = rng.random_range(100..=1000);
            
            // Deadlock detection
            while retry_count <= 3{
                // Lock from account and attempt to lock to account. Note the initial lock on from is blocking.
                let from_balance_lock = from.balance.lock();
                let to_balance_lock = to.balance.try_lock();
    
                // If both locks are acquired perform transaction and return from function.
                if to_balance_lock.is_ok(){
                    println!("Thread acquired both account locks. Processing transaction.");
                    thread::sleep(Duration::from_millis(processing_time));

                    let mut to_balance = to_balance_lock.unwrap();
                    let mut from_balance = from_balance_lock.unwrap();
    
                    match action {
                        TransactionAction::Withdraw => {
                            if *to_balance - amount < 0 {
                                panic!("Not enough funds to process request!");
                            }

                            *from_balance += amount;
                            *to_balance -= amount;

                            println!("Transferring {} from Account {:?} to Account {:?}", amount, to.id, from.id);
                        }
                        TransactionAction::Deposit => {
                            if *from_balance - amount < 0 {
                                panic!("Not enough funds to process request!");
                            }

                            *from_balance -= amount;
                            *to_balance += amount;

                            println!("Transferring {} from Account {:?} to Account {:?}", amount, from.id, to.id);
                        }
                    }
                    return;
                }

                if transaction_start.elapsed() > Duration::from_millis(1000) && retry_count < 3 {
                    retry_count += 1;
                    transaction_start = Instant::now();
                    println!("Locks could not be acquired. Retrying transaction. Attempt number: {}", retry_count);
                }
            }
            // Deadlock detection.
            println!("Locks could not be acquired. Deadlock occured cancelling transaction.");
        })
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

        let transfer_thread = transfer_service.transfer(1, 3, String::from("test"), 500);

        transfer_thread.join().unwrap();

        let transfer_accounts = transfer_service.account_data.get_account_list();
        
        assert_eq!(*transfer_accounts.get(&1).unwrap().balance.lock().unwrap(), 9500);
        assert_eq!(*transfer_accounts.get(&3).unwrap().balance.lock().unwrap(), 30500);
    }

    #[test]
    fn should_transfer_amount_in_order() {
        let transfer_service = TransferService::new();

        let transfer_thread = transfer_service.transfer(5, 1, String::from("test"), 500);

        transfer_thread.join().unwrap();

        let transfer_accounts = transfer_service.account_data.get_account_list();
        
        assert_eq!(*transfer_accounts.get(&1).unwrap().balance.lock().unwrap(), 10500);
        assert_eq!(*transfer_accounts.get(&5).unwrap().balance.lock().unwrap(), 19500);
    }

    #[test]
    #[should_panic]
    fn should_panic_when_overdraft() {
        let transfer_service = TransferService::new();

        let transfer_thread = transfer_service.transfer(1, 3, String::from("test"), 10500);

        transfer_thread.join().unwrap();
    }
}