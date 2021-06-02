use crate::model::bot::{MessageParser, GameFocusChecker};
use crate::model::key::{KeyInputtable, KeyMappable, KeyInput};
use crate::model::win_utils::{get_focused_window};
use std::str::FromStr;
use inputbot::{KeybdKey, KeybdKey::*};

#[derive(Debug)]
pub enum NDSAction {
  Up,
  Down,
  Left,
  Right,
  Y,
  X,
  A,
  B,
  L,
  R,
  Start,
  Select
}

impl FromStr for NDSAction {
  type Err = ();

  fn from_str(s: &str) -> Result<NDSAction, Self::Err> {
    match s {
      "up" => Ok(NDSAction::Up),
      "down" => Ok(NDSAction::Down),
      "left" => Ok(NDSAction::Left),
      "right" => Ok(NDSAction::Right),
      "y" => Ok(NDSAction::Y),
      "x" => Ok(NDSAction::X),
      "a" => Ok(NDSAction::A),
      "b" => Ok(NDSAction::B),
      "l" => Ok(NDSAction::L),
      "r" => Ok(NDSAction::R),
      "start" => Ok(NDSAction::Start),
      "select" => Ok(NDSAction::Select),
      _ => Err(())
    }
  }
}

impl ToString for NDSAction {
  fn to_string(&self) -> String {
    match self {
      NDSAction::Up => "up".to_string(),
      NDSAction::Down => "down".to_string(),
      NDSAction::Left => "left".to_string(),
      NDSAction::Right => "right".to_string(),
      _ => "".to_string()
    }
  }
}

pub struct NDSInput {
  action: NDSAction,
  presses: i8
}

impl KeyMappable for NDSInput {
  fn to_key_input(&self) -> Box<dyn KeyInputtable> {
    return Box::new(KeyInput::new(NDSUtils::to_keys(&self.action), self.presses, 100));
  }
}

impl KeyInputtable for NDSInput {

  fn pop(&self) -> Option<Box<dyn KeyInputtable + Send + Sync>> {
    return self.to_key_input().pop();
  }

  fn get_presses(&self) -> &i8 {
    return &self.presses;
  }
}

pub struct NDSUtils;

impl NDSUtils {
  fn to_keys(action: &NDSAction) -> Vec<KeybdKey> {
    match action {
      NDSAction::Up => vec![UpKey],
      NDSAction::Down => vec![DownKey],
      NDSAction::Left => vec![LeftKey],
      NDSAction::Right => vec![RightKey],
      NDSAction::Y => vec![AKey],
      NDSAction::X => vec![SKey],
      NDSAction::A => vec![XKey],
      NDSAction::B => vec![ZKey],
      NDSAction::L => vec![QKey],
      NDSAction::R => vec![WKey],
      NDSAction::Start => vec![EnterKey],
      NDSAction::Select => vec![OtherKey(47)]
    }
  }
}

impl MessageParser for NDSUtils {
  fn parse_msg(&self, content: &str) -> Result<Box<dyn KeyInputtable + Send + Sync>, ()> {
    let mut split = content.split(" ");
    match split.next() {
      None => return Err(()),
      Some(cmd) => {
        match NDSAction::from_str(cmd) {
          Err(_) => return Err(()),
          Ok(action) => {
            match split.next() {
              None => return Ok(Box::new(
                NDSInput { action: action, presses: 1 })),
              Some(arg) => {
                match arg.parse::<i8>() {
                  Err(_) => return Ok(Box::new(
                    NDSInput { action: action, presses: 1 })),
                  Ok(int) => return Ok(Box::new(
                    NDSInput { action: action, presses: int }))
                }
              }
            }
          }
        }
      }
    }
  }
}

impl GameFocusChecker for NDSUtils {
  fn game_focused(&self) -> bool {
    match get_focused_window().to_str() {
      None => return false,
      Some(title) => return title.contains("DeSmuME")
    }
  }
}