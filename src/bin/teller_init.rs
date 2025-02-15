use std::{thread, time::Duration};

use os3502_project1::model::teller::Teller;

fn main() {
    let teller = Teller::new();

    thread::sleep(Duration::from_secs(3));
    teller.process_customer_transactions();
}

