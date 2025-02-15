pub mod service;
pub mod model;
pub mod data;

use std::{fs::{self, File, OpenOptions}, os::unix::fs::FileTypeExt, process::Command};

const FIFO_PATH: &str = "/tmp/project1_fifo";

// Creates a fifo file in tmp folder
pub fn create_fifo() -> String {
    
    if !fs::metadata(FIFO_PATH).map(|m| m.file_type().is_fifo()).unwrap_or(false){
        Command::new("mkfifo")
        .arg(FIFO_PATH)
        .status()
        .expect("Failed to create FIFO pipe");
    }

    return String::from(FIFO_PATH);
}

// Cleans up the fifo file
pub fn delete_fifo(){
    
    if fs::metadata(FIFO_PATH).map(|m| m.file_type().is_fifo()).unwrap_or(false){
        Command::new("rm")
        .arg(FIFO_PATH)
        .status()
        .expect("Failed to delete FIFO pipe");
    }
}

// Fetches fifo file that allows writes.
pub fn get_writable_pipe() -> File {
    return OpenOptions::new()
        .write(true)
        .open(FIFO_PATH)
        .expect("Failed to open FIFO.");
}

// Fetches fifo file that allows reads.
pub fn get_readable_pipe() -> File {
    return OpenOptions::new()
        .read(true)
        .open(FIFO_PATH)
        .expect("Failed to open FIFO.");
}