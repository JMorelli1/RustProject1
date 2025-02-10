use std::{fs::OpenOptions, io::{BufRead, BufReader}, thread, time::Duration};

const FIFO_PATH: &str = "/tmp/project1_fifo";

fn main() {
    let fifo = OpenOptions::new()
    .read(true)
    .open(FIFO_PATH)
    .expect("Failed to open FIFO.");

    let reader = BufReader::new(fifo);

    for line in reader.lines(){
        let message = line.expect("Failed to read line from FIFO.");
        println!("Consumer received: {}", message);
        thread::sleep(Duration::from_millis(500));
    }
}