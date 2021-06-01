use serde::{Serialize, Deserialize};
use std::str::FromStr;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum SupportedGame {
  FTL,
  NDS
}

impl FromStr for SupportedGame {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "ftl" => Ok(SupportedGame::FTL),
      "nds" => Ok(SupportedGame::NDS),
      _ => Err(())
    }
  }
}

impl ToString for SupportedGame {

  fn to_string(&self) -> String {
    match self {
      SupportedGame::FTL => "FTL: Faster Than Light".to_string(),
      SupportedGame::NDS => "Nintendo DS".to_string()
    }
  }
}