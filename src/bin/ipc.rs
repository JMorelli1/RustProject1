use std::{env, fs, ops::Index, os::unix::fs::FileTypeExt, process::Command, thread, time::Duration};

const FIFO_PATH: &str = "/tmp/project1_fifo";


// Creates a fifo file in tmp folder
fn create_fifo(){
    
    if !fs::metadata(FIFO_PATH).map(|m| m.file_type().is_fifo()).unwrap_or(false){
        Command::new("mkfifo")
        .arg(FIFO_PATH)
        .status()
        .expect("Failed to create FIFO pipe");
    }
}

// Cleans up the fifo file
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

    thread::sleep(Duration::from_secs(20));

    // Pull program arguments
    let args: Vec<String> = env::args().collect();

    // Build file path from args
    let producer_file_relative_path = format!("{}{}", "src/assets/", args.index(1));
    let producer_file_full_path = env::current_dir().unwrap().join(producer_file_relative_path);

    /*
        Create a Producer process to output messages. This is separated to its own thread so that the entire 
        file can be written before the consumer picks it up.
    */
    let producer_thread = thread::spawn(move || {
        let mut producer = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("producer")
        .arg(producer_file_full_path)
        .spawn()
        .expect("Failed to start producer");

        producer.wait().expect("Producer process failed");
    });

    // Create a consumer process 
    let mut consumer = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("consumer")
        .spawn()
        .expect("Failed to start consumer");
        
        
    consumer.wait().expect("Consumer process failed");
    producer_thread.join().expect("Producer thread errored");

    delete_fifo();
}