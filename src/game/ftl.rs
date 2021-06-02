use crate::model::bot::{MessageParser, GameFocusChecker};
use crate::model::key::{KeyInputtable, KeyMappable, KeyInput};
use crate::model::win_utils::{get_focused_window};
use std::str::{FromStr, Split};
use inputbot::{KeybdKey, KeybdKey::*};

pub enum FTLTarget {
  Event,
  Shields,
  Engines,
  Oxygen,
  Medbay,
  CloneBay,
  Teleporter,
  Cloaking,
  MindControl,
  Hacking,
  ArtilleryBeam,
  WeaponControl,
  DroneControl,
  Doors,
  BackupBattery
}

impl FromStr for FTLTarget {
  type Err = ();

  fn from_str(s: &str) -> Result<FTLTarget, Self::Err> {
    match s {
      "event" | "choice" | "choose" => return Ok(FTLTarget::Event),
      "shields" | "shield" | "s" => return Ok(FTLTarget::Shields),
      "engines" | "engine" | "e" => return Ok(FTLTarget::Engines),
      "oxygen" | "o2" | "o" => return Ok(FTLTarget::Oxygen),
      "medbay" | "med" | "mb" => return Ok(FTLTarget::Medbay),
      "clone_bay" | "clone" | "cb" => return Ok(FTLTarget::CloneBay),
      "teleporter" | "teleport" | "tp" => return Ok(FTLTarget::Teleporter),
      "cloaking" | "cloak" | "c" => return Ok(FTLTarget::Cloaking),
      "mind_control" | "mind" | "mc" => return Ok(FTLTarget::MindControl),
      "hacking" | "hack" | "h" => return Ok(FTLTarget::Hacking),
      "artillery" | "beam" | "a" => return Ok(FTLTarget::ArtilleryBeam),
      "weapons" | "weapon" | "wep" | "w" => return Ok(FTLTarget::WeaponControl),
      "drones" | "drone" | "d" => return Ok(FTLTarget::DroneControl),
      "doors" | "door" => return Ok(FTLTarget::Doors),
      "backup" | "battery" | "b" => return Ok(FTLTarget::BackupBattery),
      _ => return Err(())
    }
  }
}

pub enum FTLAction {
  PowerShields,
  PowerEngines,
  PowerOxygen,
  PowerMedbay,
  PowerCloneBay,
  PowerTeleporter,
  PowerCloaking,
  PowerMindControl,
  PowerHacking,
  PowerArtilleryBeam,

  PowerWeapon1,
  PowerWeapon2,
  PowerWeapon3,
  PowerWeapon4,
  PowerDrone1,
  PowerDrone2,
  PowerDrone3,

  EventChoice1,
  EventChoice2,
  EventChoice3,
  EventChoice4,

  OpenDoors,
  CloseDoors,
  ActivateCloaking,
  StartHacking,
  ActivateBattery
}

pub struct FTLInput {
  action: FTLAction,
  presses: i8
}

impl KeyMappable for FTLInput {
  fn to_key_input(&self) -> Box<dyn KeyInputtable> {
    match self.presses {
      i if i >= 0 => return Box::new(
        KeyInput::new(FTLUtils::to_keys(&self.action), i, 25)
      ),
      i => {
        let mut keys: Vec<KeybdKey> = FTLUtils::to_keys(&self.action);
        keys.insert(0, LShiftKey);
        return Box::new(
          KeyInput::new(
            keys,
            i8::abs(i),
            25
          )
        )
      }
    }
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
      FTLAction::PowerShields => vec![AKey],
      FTLAction::PowerEngines => vec![SKey],
      FTLAction::PowerOxygen => vec![FKey],
      FTLAction::PowerMedbay => vec![DKey],
      FTLAction::PowerCloneBay => vec![DKey],
      FTLAction::PowerTeleporter => vec![GKey],
      FTLAction::PowerCloaking => vec![HKey],
      FTLAction::PowerMindControl => vec![KKey],
      FTLAction::PowerHacking => vec![LKey],
      FTLAction::PowerArtilleryBeam => vec![YKey],
      // This is tedious as fuck
  
      FTLAction::PowerWeapon1 | FTLAction::EventChoice1 => vec![Numrow1Key],
      FTLAction::PowerWeapon2 | FTLAction::EventChoice2 => vec![Numrow2Key],
      FTLAction::PowerWeapon3 | FTLAction::EventChoice3 => vec![Numrow3Key],
      FTLAction::PowerWeapon4 | FTLAction::EventChoice4 => vec![Numrow4Key],
      FTLAction::PowerDrone1 => vec![Numrow5Key],
      FTLAction::PowerDrone2 => vec![Numrow6Key],
      FTLAction::PowerDrone3 => vec![Numrow7Key],
  
      FTLAction::OpenDoors => vec![ZKey],
      FTLAction::CloseDoors => vec![XKey],
      FTLAction::ActivateCloaking => vec![CKey],
      FTLAction::StartHacking => vec![NKey],
      FTLAction::ActivateBattery => vec![BKey]
    }
  }

  fn parse_action(&self, target: FTLTarget, arg: Option<&str>) -> Result<FTLAction, &str> {
    match target {
      FTLTarget::Shields => {
        return self.parse_system_arg(FTLAction::PowerShields, arg);
      },
      FTLTarget::Engines => {
        return self.parse_system_arg(FTLAction::PowerEngines, arg);
      },
      FTLTarget::Oxygen => {
        return self.parse_system_arg(FTLAction::PowerOxygen, arg);
      },
      FTLTarget::Medbay => {
        return self.parse_system_arg(FTLAction::PowerMedbay, arg);
      },
      FTLTarget::CloneBay => {
        return self.parse_system_arg(FTLAction::PowerCloneBay, arg);
      },
      FTLTarget::Teleporter => {
        return self.parse_system_arg(FTLAction::PowerTeleporter, arg);
      },
      // Cloaking is an exception.
      FTLTarget::Cloaking => {
        return self.parse_startable_system_arg(
          FTLAction::ActivateCloaking,
          FTLAction::PowerCloaking,
          arg
        );
      },
      FTLTarget::MindControl => {
        return self.parse_system_arg(FTLAction::PowerMindControl, arg);
      },
      // Hacking is also an exception.
      FTLTarget::Hacking => {
        return self.parse_startable_system_arg(
          FTLAction::StartHacking,
          FTLAction::PowerHacking,
          arg
        );
      }
      FTLTarget::ArtilleryBeam => {
        return self.parse_system_arg(FTLAction::PowerArtilleryBeam, arg);
      }
      FTLTarget::Event => { return self.parse_event_arg(arg); },
      FTLTarget::WeaponControl => { return self.parse_weapon_arg(arg); },
      FTLTarget::DroneControl => { return self.parse_drone_arg(arg); },
      FTLTarget::Doors => { return self.parse_doors_arg(arg); },
      FTLTarget::BackupBattery => { return Ok(FTLAction::ActivateBattery); }
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
          1 => return Ok(FTLAction::EventChoice1),
          2 => return Ok(FTLAction::EventChoice2),
          3 => return Ok(FTLAction::EventChoice3),
          4 => return Ok(FTLAction::EventChoice4),
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
          1 => return Ok(FTLAction::PowerWeapon1),
          2 => return Ok(FTLAction::PowerWeapon2),
          3 => return Ok(FTLAction::PowerWeapon3),
          4 => return Ok(FTLAction::PowerWeapon4),
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
          1 => return Ok(FTLAction::PowerDrone1),
          2 => return Ok(FTLAction::PowerDrone2),
          3 => return Ok(FTLAction::PowerDrone3),
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
        "open" => return Ok(FTLAction::OpenDoors),
        "close" => return Ok(FTLAction::CloseDoors),
        _ => return doors_err
      }
    }
  }

  fn is_power_action(&self, action: &FTLAction) -> bool {
    match action {
      FTLAction::PowerShields | FTLAction::PowerEngines |
      FTLAction::PowerOxygen | FTLAction::PowerMedbay |
      FTLAction::PowerCloneBay | FTLAction::PowerTeleporter |
      FTLAction::PowerCloaking | FTLAction::PowerMindControl |
      FTLAction::PowerHacking | FTLAction::PowerArtilleryBeam => return true,
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
            Err(_) => return Err(()),
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