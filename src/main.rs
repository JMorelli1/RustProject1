mod model;
mod service;
mod data;

use std::{process::Command, thread, vec};
use os3502_project1::{create_fifo, delete_fifo, get_writable_pipe, get_readable_pipe};

fn main() {
    create_fifo();

    let customer_start_up = vec![
        ("Jim", 1),
        ("Kelly", 4),
        ("Frank", 9),
        ("Morgan", 10),
        ("Teddy", 3),
        ("Joesph", 6),
        ("Jessica", 7),
        ("Albert", 2),
        ("Tyler", 5),
        ("Linda", 8),
        ("Jim", 1),
        ("Kelly", 4),
        ("Frank", 9),
        ("Morgan", 10),
        ("Teddy", 3),
        ("Joesph", 6),
        ("Jessica", 7),
        ("Albert", 2),
        ("Tyler", 5),
        ("Linda", 8),
    ];

    let mut customer_threads = vec![];

    for customer in customer_start_up {
        let customer_thread = thread::spawn(move || {
            let mut customer_process = Command::new("cargo")
            .arg("run")
            .arg("--bin")
            .arg("customer_init")
            .arg(customer.0)
            .arg(customer.1.to_string())
            .spawn()
            .expect("Failed to start customer action");
    
            customer_process.wait().expect("Producer process failed");
        });

        customer_threads.push(customer_thread);
    }

    let mut teller_process = Command::new("cargo")
    .arg("run")
    .arg("--bin")
    .arg("teller_init")
    .spawn()
    .expect("Failed to start consumer");

    teller_process.wait().expect("Consumer failed");
    for customer_thread in customer_threads {
        customer_thread.join().expect("Customer thread failed.");
    }

    delete_fifo();
}
