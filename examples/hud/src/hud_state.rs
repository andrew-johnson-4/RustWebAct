use std::cmp::{max,min};

pub struct CharacterProfile {
   pub health: (u64,u64),
   pub energy: (u64,u64),
   pub mana: (u64,u64)
}
impl CharacterProfile {
   pub fn regen_health(&mut self, v: u64) {
      self.health.0 = min(self.health.0+v, self.health.1);
   }
   pub fn regen_energy(&mut self, v: u64) {
      self.energy.0 = min(self.energy.0+v, self.energy.1);
   }
   pub fn regen_mana(&mut self, v: u64) {
      self.mana.0 = min(self.mana.0+v, self.mana.1);
   }
}

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
