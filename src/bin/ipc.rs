use std::{fs, os::unix::fs::FileTypeExt, process::Command, thread, time::Duration};

const FIFO_PATH: &str = "/tmp/project1_fifo";

fn create_fifo(){
    
    if !fs::metadata(FIFO_PATH).map(|m| m.file_type().is_fifo()).unwrap_or(false){
        Command::new("mkfifo")
        .arg(FIFO_PATH)
        .status()
        .expect("Failed to create FIFO pipe");
    }
}

fn delete_fifo(){
    
    if fs::metadata(FIFO_PATH).map(|m| m.file_type().is_fifo()).unwrap_or(false){
        Command::new("rm")
        .arg(FIFO_PATH)
        .status()
        .expect("Failed to delete FIFO pipe");
    }
}

fn main() {
    create_fifo();

    let mut producer = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("producer")
        .spawn()
        .expect("Failed to start producer");

    

    thread::sleep(Duration::from_millis(2000));
    let mut consumer = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("consumer")
        .spawn()
        .expect("Failed to start consumer");
        
        
    consumer.wait().expect("Consumer process failed");
    producer.wait().expect("Producer process failed");

    delete_fifo();
}