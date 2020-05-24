use std::cmp::{max,min};

pub struct ChatLog {
   pub input: String,
   pub show: String,
   pub channels: Vec<(String,Vec<String>)>
}
impl ChatLog {
    pub fn new() -> ChatLog {
       ChatLog {
          input: String::new(),
          show: "local".to_string(),
          channels: vec![
             ("local".to_string(), Vec::new()),
             ("character".to_string(), Vec::new()),
          ]
       }
    }
}
