use std::{collections::HashMap, sync::Arc};

use crate::{model::Account, service::transaction_action::TransactionAction};


/*
This is essential just for dummy data.
Contains a hashmap of generated Accounts with a sequential key to use as the access order for locks.
 */ 
pub struct AccountsData{
    accounts: HashMap<i32, Arc<Account>>
}

impl AccountsData {
    pub fn new() -> Arc<Self>{

        let accounts: Vec<Arc<Account>> = vec![
            Account::new(1, 10000),
            Account::new(2, 20000),
            Account::new(3, 30000),
            Account::new(4, 40000),
            Account::new(5, 20000),
            Account::new(6, 10000),
            Account::new(7, 50000),
            Account::new(8, 80000),
            Account::new(9, 50000),
            Account::new(10, 40000),
        ];

        let accounts_map: HashMap<i32, Arc<Account>> = accounts.into_iter()
            .map(|account| (account.id, account))
            .collect();

        Arc::new(Self{
            accounts: accounts_map
        })
    }


    // Returns the asked for accounts in the order they should be locked. Attached the action to process so the funds flow the proper direction.
    pub fn fetch_accounts(&self, from:i32, to:i32) -> (Arc<Account>, Arc<Account>, TransactionAction){
        let from_account = self.accounts.get(&from).expect("Failed to find account").clone();
        let to_account = self.accounts.get(&to).expect("Failed to find account").clone();

        if from < to {
            return (from_account, to_account, TransactionAction::Deposit);
        }
        else if from > to {
            return (to_account, from_account, TransactionAction::Withdraw);
        }
        else {
            panic!("Accounts can not match.");
        }
    }

    pub fn get_account_list(&self) -> HashMap<i32, Arc<Account>> {
        return self.accounts.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_deposit_order() {
        let account_data = AccountsData::new();

        let (account1, account2, action) = account_data.fetch_accounts(1, 2);
        
        assert_eq!(account1.id, 1);
        assert_eq!(account2.id, 2);
        assert_eq!(action, TransactionAction::Deposit);
    }

    #[test]
    fn should_return_withdraw_order() {
        let account_data = AccountsData::new();

        let (account1, account2, action) = account_data.fetch_accounts(2, 1);
        
        assert_eq!(account1.id, 1);
        assert_eq!(account2.id, 2);
        assert_eq!(action, TransactionAction::Withdraw);
    }

    #[test]
    #[should_panic]
    fn should_panic_when_accounts_match() {
        let account_data = AccountsData::new();

        account_data.fetch_accounts(2, 2);
    }
}