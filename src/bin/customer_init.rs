use std::{env, ops::Index};

use os3502_project1::model::Client;

fn main() {
    // Pull program arguments
    let args: Vec<String> = env::args().collect();

    let client = Client::new(String::from(args.index(1)), args.index(2).parse().unwrap());

    client.start_request();
}