use serde::{Serialize, Deserialize};
use std::{
    collections::HashMap,
    fs::read_to_string,
};

#[derive(Deserialize, Serialize)]
pub struct Dictionary(HashMap<String, String>);

impl Dictionary {

    pub fn from_json(path: &str) -> Self {
        let file = read_to_string(path).expect("Could not ready JSON");
        let dic: Dictionary  = serde_json::from_str(file.as_str())
            .expect("Could not deserialize dictionary JSON");
        // println!("{:?}", serde_json::to_string_pretty(&dic));
        dic
    }

    // TODO less naive implementation
    pub fn get_def(&self, word: &str) -> Option<&String> {
        self.0.get(word)
    }
}
