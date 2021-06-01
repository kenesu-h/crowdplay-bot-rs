use crate::model::bot::{MessageParser, GameFocusChecker};
use crate::model::key::{KeyInputtable, KeyMappable, KeyInput};
use crate::model::win_utils::{get_focused_window};
use std::str::FromStr;
use std::mem::ManuallyDrop;
use inputbot::{KeybdKey, KeybdKey::*};

pub enum FTLTarget {
  EVENT,
  SHIELDS,
  ENGINES,
  OXYGEN,
  MEDBAY,
  CLONE_BAY,
  TELEPORTER,
  CLOAKING,
  MIND_CONTROL,
  HACKING,
  ARTILLERY_BEAM,
  WEAPON_CONTROL,
  DRONE_CONTROL,
  DOORS,
  BACKUP_BATTERY
}

impl FromStr for FTLTarget {
  type Err = ();

  fn from_str(s: &str) -> Result<FTLTarget, Self::Err> {
    match s {
      "event" | "choice" | "choose" => return Ok(FTLTarget::EVENT),
      "shields" | "shield" | "s" => return Ok(FTLTarget::SHIELDS),
      "engines" | "engine" | "e" => return Ok(FTLTarget::ENGINES),
      "oxygen" | "o2" | "o" => return Ok(FTLTarget::OXYGEN),
      "medbay" | "med" | "mb" => return Ok(FTLTarget::MEDBAY),
      "clone_bay" | "clone" | "cb" => return Ok(FTLTarget::CLONE_BAY),
      "teleporter" | "teleport" | "tp" => return Ok(FTLTarget::TELEPORTER),
      "cloaking" | "cloak" | "c" => return Ok(FTLTarget::CLOAKING),
      "mind_control" | "mind" | "mc" => return Ok(FTLTarget::MIND_CONTROL),
      "hacking" | "hack" | "h" => return Ok(FTLTarget::HACKING),
      "artillery" | "beam" | "a" => return Ok(FTLTarget::ARTILLERY_BEAM),
      "weapons" | "weapon" | "wep" | "w" => return Ok(FTLTarget::WEAPON_CONTROL),
      "drones" | "drone" | "d" => return Ok(FTLTarget::DRONE_CONTROL),
      "doors" | "door" => return Ok(FTLTarget::DOORS),
      "backup" | "battery" | "b" => return Ok(FTLTarget::BACKUP_BATTERY)
    }
  }
}

pub enum FTLAction {
  POWER_SHIELDS,
  POWER_ENGINES,
  POWER_OXYGEN,
  POWER_MEDBAY,
  POWER_CLONE_BAY,
  POWER_TELEPORTER,
  POWER_CLOAKING,
  POWER_MIND_CONTROL,
  POWER_HACKING,
  POWER_ARTILLERY_BEAM,

  POWER_WEAPON_1,
  POWER_WEAPON_2,
  POWER_WEAPON_3,
  POWER_WEAPON_4,
  POWER_DRONE_1,
  POWER_DRONE_2,
  POWER_DRONE_3,

  EVENT_CHOICE_1,
  EVENT_CHOICE_2,
  EVENT_CHOICE_3,
  EVENT_CHOICE_4,

  OPEN_DOORS,
  CLOSE_DOORS,
  ACTIVATE_CLOAKING,
  START_HACKING,
  START_MIND_CONTROL,
  ACTIVATE_BATTERY
}

fn to_keys(action: &Option<FTLAction>) -> Vec<KeybdKey> {
  match action {
    None => Vec::new(),
    Some(action) => match action {
      FTLAction::POWER_SHIELDS => vec![AKey],
      FTLAction::POWER_ENGINES => vec![SKey],
      FTLAction::POWER_OXYGEN => vec![FKey],
      FTLAction::POWER_MEDBAY => vec![DKey],
      FTLAction::POWER_CLONE_BAY => vec![DKey],
      FTLAction::POWER_TELEPORTER => vec![GKey],
      FTLAction::POWER_CLOAKING => vec![HKey],
      FTLAction::POWER_MIND_CONTROL => vec![KKey],
      FTLAction::POWER_HACKING => vec![LKey],
      FTLAction::POWER_ARTILLERY_BEAM => vec![YKey],
      // This is tedious as fuck

      FTLAction::POWER_WEAPON_1 | FTLAction::EVENT_CHOICE_1 => vec![Numrow1Key],
      FTLAction::POWER_WEAPON_2 | FTLAction::EVENT_CHOICE_2 => vec![Numrow2Key],
      FTLAction::POWER_WEAPON_3 | FTLAction::EVENT_CHOICE_3 => vec![Numrow3Key],
      FTLAction::POWER_WEAPON_4 | FTLAction::EVENT_CHOICE_4 => vec![Numrow4Key],
      FTLAction::POWER_DRONE_1 => vec![Numrow5Key],
      FTLAction::POWER_DRONE_2 => vec![Numrow6Key],
      FTLAction::POWER_DRONE_3 => vec![Numrow7Key],

      FTLAction::OPEN_DOORS => vec![ZKey],
      FTLAction::CLOSE_DOORS => vec![XKey],
      FTLAction::ACTIVATE_CLOAKING => vec![CKey],
      FTLAction::START_HACKING => vec![NKey],
      FTLAction::START_MIND_CONTROL => vec![MKey],
      FTLAction::ACTIVATE_BATTERY => vec![BKey]
    }
  }
}

pub struct FTLInput {
  action: Option<FTLAction>,
  presses: i8
}

impl KeyMappable for FTLInput {
  fn to_key_input(&self) -> Box<dyn KeyInputtable> {
    return Box::new(KeyInput::new(to_keys(&self.action), self.presses));
  }
}

impl KeyInputtable for FTLInput {

  fn pop(&self) -> Option<Box<dyn KeyInputtable + Send + Sync>> {
    return self.to_key_input().pop();
  }

  fn get_presses(&self) -> &i8 {
    return &self.presses;
  }
}

pub struct FTLUtils;

#[repr(C)]
union FTLActionArg {
  s: ManuallyDrop<String>,
  i: i8 
}

impl FTLUtils {
  fn parse_action(&self, target: FTLTarget, arg: Option<String>) -> Result<FTLAction, String> {
    let system_err: Result<FTLAction, String> =
      Err("System power allocation must be accompanied by an integer whose
      absolute value is in the range [1, 8].".to_string());
    match target {
      FTLTarget::EVENT => {
        let event_err: Result<FTLAction, String> =
          Err("Event choice must be accompanied by an integer within the range 
          [1, 4].".to_string());
        match arg {
          None => return event_err,
          Some(action_arg) => match action_arg.parse::<i8>() {
            Err(_) => return event_err,
            Ok(i) => match i {
              1 => return Ok(FTLAction::EVENT_CHOICE_1),
              2 => return Ok(FTLAction::EVENT_CHOICE_2),
              3 => return Ok(FTLAction::EVENT_CHOICE_3),
              4 => return Ok(FTLAction::EVENT_CHOICE_4),
              _ => return event_err
            }
          }
        }
      },
      FTLTarget::SHIELDS => {
        return self.parse_system_arg(FTLAction::POWER_SHIELDS, arg)
      },
      FTLTarget::ENGINES => {
        return self.parse_system_arg(FTLAction::POWER_ENGINES, arg)
      },
      FTLTarget::OXYGEN => {
        return self.parse_system_arg(FTLAction::POWER_OXYGEN, arg)
      }
      
    }
  }

  fn parse_system_arg(&self, action: FTLAction, arg: Option<String>) -> Result<FTLAction, String> {
    let system_err: Result<FTLAction, String> =
      Err("System power allocation must be accompanied by an integer whose
      absolute value is in the range [1, 8].".to_string());
    match arg {
      None => return system_err,
      Some(action_arg) => match action_arg.parse::<i8>() {
        Err(_) => system_err,
        Ok(i) => {
          let a: i8 = i8::abs(i);
          if a >= 1 && a <= 8 {
            return Ok(action);
          } else {
            return system_err;
          }
        }
      }
    }
  }
}

impl MessageParser for FTLUtils {
  fn parse_msg(&self, content: &str) -> Result<Box<dyn KeyInputtable + Send + Sync>, ()> {
    return Err(());
  }


}