use std::{io::Write, os::fd::AsRawFd, sync::Arc};

use nix::fcntl::{flock, FlockArg};
use rand::Rng;

use crate::get_writable_pipe;

pub struct Client {
    name: String,
    account_id: i32,
}

impl Client {
    pub fn new(name: String, account_id: i32) -> Arc<Self> {
        Arc::new(Self {
            name,
            account_id
        })
    }

    /*
        Opens fifo file and locks the file to write to.
        Generates a random amount to transfer to another account.
    */
    pub fn start_request(&self) {
        let fifo = get_writable_pipe();

        flock(fifo.as_raw_fd(), FlockArg::LockExclusive).expect("Failed to lock FIFO file.");

        let mut rng = rand::rng();
        let to_account = self.generate_random_account();
        
        let amount = rng.random_range(100..=1000);
        println!("Customer {} requesting transaction of account id {} to {}", self.name, self.account_id, to_account);

        writeln!(&fifo, "{},{},{},{}", self.name, self.account_id, to_account, amount).expect("Failed to write client request.");

        flock(fifo.as_raw_fd(), FlockArg::Unlock).expect("Failed to unlock the FIFO file");
    }

    fn generate_random_account(&self) -> i32 {
        let mut rng = rand::rng();

        // Generate random account to deposit money into.
        let mut account_id = rng.random_range(1..=10);

        // Reroll account number if it matches customers account id.
        while account_id == self.account_id {
            account_id = rng.random_range(1..=10);
        }

        return account_id;
    }
}

