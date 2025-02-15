use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Account {
    pub id: i32,
    pub balance: Mutex<i32>,
}

impl Account {
pub fn new(id: i32, balance: i32) -> Arc<Self> {
        Arc::new(Self {
            id,
            balance: Mutex::new(balance)
        })
    }
}