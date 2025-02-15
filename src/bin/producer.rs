use std::{env::{self}, fs::{File, OpenOptions}, io::{self, BufRead, BufWriter, Write}, ops::Index};

const FIFO_PATH: &str = "/tmp/project1_fifo";

// Producer will output file line by line to the named pipe.
fn main() {
    let fifo = OpenOptions::new()
    .write(true)
    .open(FIFO_PATH)
    .expect("Failed to open FIFO.");

    // Pull program arguments
    let args: Vec<String> = env::args().collect();
    let mut writer = BufWriter::new(fifo);

    // Read file from path provided
    if let Ok(file) = File::open(&args.index(1)) {
        let reader = io::BufReader::new(file);

        for (index, line) in reader.lines().enumerate() {
            match line {
                Ok(content) => {
                    // Log message and write to FIFO file
                    println!("Producer writing {}", content);
                    writeln!(writer, "{}", content).expect("Failed to write to FIFO");
                }
                Err(err) => eprintln!("Error reading line from file. Line: {} {}", index+1, err)
            }
        }
    }
}