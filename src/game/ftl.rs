use crate::model::bot::{MessageParser, GameFocusChecker};
use crate::model::key::{KeyInputtable, KeyMappable, KeyInput};
use crate::model::win_utils::{get_focused_window};
use std::str::{FromStr, Split};
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
      "backup" | "battery" | "b" => return Ok(FTLTarget::BACKUP_BATTERY),
      _ => return Err(())
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
  // START_MIND_CONTROL,
  ACTIVATE_BATTERY
}

pub struct FTLInput {
  action: FTLAction,
  presses: i8
}

impl KeyMappable for FTLInput {
  fn to_key_input(&self) -> Box<dyn KeyInputtable> {
    return Box::new(KeyInput::new(FTLUtils::to_keys(&self.action), self.presses));
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

impl FTLUtils {
  fn to_keys(action: &FTLAction) -> Vec<KeybdKey> {
    match action {
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
      // FTLAction::START_MIND_CONTROL => vec![MKey],
      FTLAction::ACTIVATE_BATTERY => vec![BKey]
    }
  }

  fn parse_action(&self, target: FTLTarget, arg: Option<&str>) -> Result<FTLAction, &str> {
    match target {
      FTLTarget::SHIELDS => {
        return self.parse_system_arg(FTLAction::POWER_SHIELDS, arg);
      },
      FTLTarget::ENGINES => {
        return self.parse_system_arg(FTLAction::POWER_ENGINES, arg);
      },
      FTLTarget::OXYGEN => {
        return self.parse_system_arg(FTLAction::POWER_OXYGEN, arg);
      },
      FTLTarget::MEDBAY => {
        return self.parse_system_arg(FTLAction::POWER_MEDBAY, arg);
      },
      FTLTarget::CLONE_BAY => {
        return self.parse_system_arg(FTLAction::POWER_CLONE_BAY, arg);
      },
      FTLTarget::TELEPORTER => {
        return self.parse_system_arg(FTLAction::POWER_TELEPORTER, arg);
      },
      // Cloaking is an exception.
      FTLTarget::CLOAKING => {
        return self.parse_startable_system_arg(
          FTLAction::ACTIVATE_CLOAKING,
          FTLAction::POWER_CLOAKING,
          arg
        );
      },
      FTLTarget::MIND_CONTROL => {
        return self.parse_system_arg(FTLAction::POWER_MIND_CONTROL, arg);
      },
      // Hacking is also an exception.
      FTLTarget::HACKING => {
        return self.parse_startable_system_arg(
          FTLAction::START_HACKING,
          FTLAction::POWER_HACKING,
          arg
        );
      }
      FTLTarget::ARTILLERY_BEAM => {
        return self.parse_system_arg(FTLAction::POWER_ARTILLERY_BEAM, arg);
      }
      FTLTarget::EVENT => { return self.parse_event_arg(arg); },
      FTLTarget::WEAPON_CONTROL => { return self.parse_weapon_arg(arg); },
      FTLTarget::DRONE_CONTROL => { return self.parse_drone_arg(arg); },
      FTLTarget::DOORS => { return self.parse_doors_arg(arg); },
      FTLTarget::BACKUP_BATTERY => { return Ok(FTLAction::ACTIVATE_BATTERY); }
    }
  }

  fn parse_system_arg(&self, action: FTLAction, arg: Option<&str>) -> Result<FTLAction, &str> {
    let power_err: Result<FTLAction, &str> =
      Err("System power allocation must be accompanied by an integer whose
      absolute value is in the range [1, 8].");
    match arg {
      None => return power_err,
      Some(power_arg) => match power_arg.parse::<i8>() {
        Err(_) => power_err,
        Ok(i) => {
          let a: i8 = i8::abs(i);
          if a >= 1 && a <= 8 {
            return Ok(action);
          } else {
            return power_err;
          }
        }
      }
    }
  }

  fn parse_startable_system_arg(&self, start_action: FTLAction, power_action: FTLAction, arg: Option<&str>) -> Result<FTLAction, &str> {
    let power_err: Result<FTLAction, &str> =
      Err("System power allocation must be accompanied by an integer whose
      absolute value is in the range [1, 8].");
    match arg {
      None => return Ok(start_action),
      Some(power_arg) => match power_arg.parse::<i8>() {
        Err(_) => power_err,
        Ok(i) => {
          let a: i8 = i8::abs(i);
          if a >= 1 && a <= 8 {
            return Ok(power_action);
          } else {
            return power_err;
          }
        }
      }
    }
  }

  fn parse_event_arg(&self, arg: Option<&str>) -> Result<FTLAction, &str> {
    let event_err: Result<FTLAction, &str> =
      Err("Event choice must be accompanied by an integer within the range 
      [1, 4].");
    match arg {
      None => return event_err,
      Some(choice_arg) => match choice_arg.parse::<i8>() {
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
  }

  fn parse_weapon_arg(&self, arg: Option<&str>) -> Result<FTLAction, &str> {
    let weapon_err: Result<FTLAction, &str> =
      Err("Weapon control must be accompanied by an integer within the range
      [1, 4].");
    match arg {
      None => return weapon_err,
      Some(choice_arg) => match choice_arg.parse::<i8>() {
        Err(_) => return weapon_err,
        Ok(i) => match i {
          1 => return Ok(FTLAction::POWER_WEAPON_1),
          2 => return Ok(FTLAction::POWER_WEAPON_2),
          3 => return Ok(FTLAction::POWER_WEAPON_3),
          4 => return Ok(FTLAction::POWER_WEAPON_4),
          _ => return weapon_err
        }
      }
    }
  }

  fn parse_drone_arg(&self, arg: Option<&str>) -> Result<FTLAction, &str> {
    let drone_err: Result<FTLAction, &str> =
      Err("Drone control must be accompanied by an integer within the range
      [1, 3].");
    match arg {
      None => return drone_err,
      Some(choice_arg) => match choice_arg.parse::<i8>() {
        Err(_) => return drone_err,
        Ok(i) => match i {
          1 => return Ok(FTLAction::POWER_DRONE_1),
          2 => return Ok(FTLAction::POWER_DRONE_2),
          3 => return Ok(FTLAction::POWER_DRONE_3),
          _ => return drone_err
        }
      }
    }
  }

  fn parse_doors_arg(&self, arg: Option<&str>) -> Result<FTLAction, &str> {
    let doors_err: Result<FTLAction, &str> =
      Err("Door control must be accompanied by either \"open\" or \"close\".");
    match arg {
      None => return doors_err,
      Some(doors_arg) => match doors_arg {
        "open" => return Ok(FTLAction::OPEN_DOORS),
        "close" => return Ok(FTLAction::CLOSE_DOORS),
        _ => return doors_err
      }
    }
  }

  fn is_power_action(&self, action: &FTLAction) -> bool {
    match action {
      FTLAction::POWER_SHIELDS | FTLAction::POWER_ENGINES |
      FTLAction::POWER_OXYGEN | FTLAction::POWER_MEDBAY |
      FTLAction::POWER_CLONE_BAY | FTLAction::POWER_TELEPORTER |
      FTLAction::POWER_CLOAKING | FTLAction::POWER_MIND_CONTROL |
      FTLAction::POWER_HACKING | FTLAction::POWER_ARTILLERY_BEAM => return true,
      _ => return false
    }
  }
}

impl MessageParser for FTLUtils {
  fn parse_msg(&self, content: &str) -> Result<Box<dyn KeyInputtable + Send + Sync>, ()> {
    let mut split: Split<&str> = content.split(" ");
    match split.next() {
      None => return Err(()),
      Some(cmd) => match FTLTarget::from_str(cmd) {
        Err(_) => return Err(()),
        Ok(target) => {
          let arg: Option<&str> = split.next();
          let parsed: Result<FTLAction, &str> = self.parse_action(target, arg);
          match parsed {
            Err(e) => return Err(()),
            Ok(action) if self.is_power_action(&action) => {
              return Ok(Box::new(
                FTLInput {
                  action: action,
                  presses: arg.unwrap().parse::<i8>().unwrap()
                }
              ));
            },
            Ok(action) => {
              return Ok(Box::new(
                FTLInput {
                  action: action,
                  presses: 1
                }
              ));
            }
          }
        }
      }
    }
  }
}

impl GameFocusChecker for FTLUtils {
  fn game_focused(&self) -> bool {
    match get_focused_window().to_str() {
      None => return false,
      Some(title) => return title == "FTL: Faster Than Light"
    }
  }
}