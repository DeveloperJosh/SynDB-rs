use serde_json::{Result, Value};
use serde_json::json;
use std::fs;

pub struct Database {
    pub data: Value,
    pub filename: String,
    pub save_on_exit: bool,
}


impl Database { 
    pub fn new(filename: &str, save_on_exit: bool) -> Database {
        Database {
            data: Value::Null,
            filename: filename.to_string(),
            save_on_exit: save_on_exit,
        }
    }

    pub fn load(&mut self) {
        let contents = fs::read_to_string(&self.filename).expect("Something went wrong reading the file");
        self.data = serde_json::from_str(&contents).expect("Something went wrong parsing the file");
    }

    pub fn set(&mut self, key: &str, value: &str) {
        // set data like {key: value}
        self.data.as_object_mut().unwrap().insert(key.to_string(), json!(value));
    }
    pub fn get(&self, key: &str) -> String {
        // get data like {key: value}
        self.data.as_object().unwrap().get(key).unwrap().as_str().unwrap().to_string()
    }
    pub fn save(&self) {
        let data = serde_json::to_string(&self.data).expect("Something went wrong serializing the data");
        fs::write(&self.filename, data).expect("Something went wrong writing the data");
    }
}