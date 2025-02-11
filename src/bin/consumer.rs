use std::{fs::OpenOptions, io::{BufRead, BufReader}};

const FIFO_PATH: &str = "/tmp/project1_fifo";

// Consumer pulls message from FIFO file, converts it to upper case, and logs it to the console.
fn main() {
    let fifo = OpenOptions::new()
    .read(true)
    .open(FIFO_PATH)
    .expect("Failed to open FIFO.");

    let reader = BufReader::new(fifo);

    for line in reader.lines(){
        let message = line.expect("Failed to read line from FIFO.");
        let formatted_message = format!("Consumer processed: {}", message.to_uppercase());

        println!("{}", formatted_message);
    }
}