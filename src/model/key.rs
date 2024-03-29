use inputbot::KeybdKey;
use std::{
  convert::TryFrom,
  thread::sleep,
  time::Duration
};

pub trait KeyInputtable {

  // I had to take a bit of a functional approach here since my understanding
  // of mutability in Rust is still pretty rough.
  // Executes the input once, then returns the next input. The next input is
  // effectively the same input, but with one less button press.
  fn pop(&self) -> Option<Box<dyn KeyInputtable + Send + Sync>>;

  fn get_presses(&self) -> &i8;
}

pub trait KeyMappable {
  fn to_key_input(&self) -> Box<dyn KeyInputtable>;
}

pub struct KeyInput {
  keys: Vec<KeybdKey>,
  presses: i8,
  delay: i16
}

impl KeyInput {
  pub fn new(keys: Vec<KeybdKey>, presses: i8, delay: i16) -> KeyInput {
    KeyInput {
      keys: keys,
      presses: presses,
      delay: delay
    }
  }
}

impl KeyInputtable for KeyInput {

  fn pop(&self) -> Option<Box<dyn KeyInputtable + Send + Sync>> {
    match self.presses {
      0 => return None,
      _ => {
        let converted_delay: u64 = u64::try_from(self.delay).unwrap();
        for i in 0..self.keys.len() {
          self.keys[i].press();
          sleep(Duration::from_millis(converted_delay));
        }
        // 75ms is good for Pokemon
        // sleep(Duration::from_millis(75));
        for i in 0..self.keys.len() {
          self.keys[i].release();
          // sleep(Duration::from_millis(converted_delay));
        }
        // sleep(Duration::from_millis(75));
        return Some(Box::new(
          KeyInput::new(
            self.keys.clone(), 
            self.presses - 1, 
            self.delay
          )
        ));
      }
    }
  }

  fn get_presses(&self) -> &i8 {
    return &self.presses;
  }
}