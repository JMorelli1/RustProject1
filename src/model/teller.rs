use std::{io::{BufRead, BufReader}, sync::Arc};

use crate::{get_readable_pipe, service::transfer_service::TransferService};

pub struct Teller{
    transfer_service: TransferService,
}

impl Teller {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            transfer_service: TransferService::new(),
        })
    }

    // Gets fifo file and reads customer requests. Starts a thread to process the customer transfer.
    pub fn process_customer_transactions(&self) {
        let fifo = get_readable_pipe();
        let reader = BufReader::new(fifo);
        let mut transactions = vec![];

        for line in reader.lines(){
            let message = line.expect("Failed to read line from FIFO.");
            
            let message_values: Vec<&str> = message.split(",").collect();
            let (customer_name, from_account, to_account, amount) = (message_values[0], message_values[1], message_values[2], message_values[3]);

            let transfer_thread = self.transfer_service.start_transfer(from_account.parse().unwrap(), to_account.parse().unwrap(), String::from(customer_name), amount.parse().unwrap());

            transactions.push(transfer_thread);
        }    

        for transaction in transactions {
            transaction.join().unwrap();
        }

        self.transfer_service.print_final_accounts();
        
    }
}