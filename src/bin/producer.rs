use std::{fs::OpenOptions, io::Write, thread, time::Duration};

const FIFO_PATH: &str = "/tmp/project1_fifo";

fn main() {
    let mut fifo = OpenOptions::new()
    .write(true)
    .open(FIFO_PATH)
    .expect("Failed to open FIFO.");

    let messages = [
        "First message from producer",
        "Second message from producer",
        "Third message from producer"
    ];

    for msg in messages {
        println!("Producer writing {}", msg);
        writeln!(fifo, "{}", msg).expect("Failed to write to FIFO");
        thread::sleep(Duration::from_millis(500));
    }
}