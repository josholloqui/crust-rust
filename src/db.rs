use std::fs::File;
use std::sync::Arc;

use serde_json::from_reader;
use tokio::sync::Mutex;

use crate::models::Business;

pub type Db = Arc<Mutex<Vec<Business>>>;

pub fn init_db() -> Db {
    let file = File::open("../data/business.json");
    match file {
        Ok(json) => {
            let businesses = from_reader(json).unwrap();
            Arc::new(Mutex::new(businesses))
        },
        Err(_) => {
            Arc::new(Mutex::new(Vec::new()))
        }
    }
}