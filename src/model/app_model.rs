use crate::model::bot::{Bot, BotSerenity, GameFocusChecker, MessageParser};
use crate::game::{
  ftl::FTLUtils,
  nds::NDSUtils,
  supported_game::{SupportedGame}
};
use serde::{Serialize, Deserialize};
use serenity::async_trait;

#[async_trait]
pub trait AppModel {
  async fn start(&self) -> Result<(), String>;

  async fn build_bot(&self) -> Result<Box<dyn Bot + Send + Sync>, &str>;

  fn get_prefix(&self) -> &String;

  fn set_prefix(&self, prefix: String) -> Box<dyn AppModel>;

  fn get_token(&self) -> &String;

  fn set_token(&self, token: String) -> Box<dyn AppModel>;

  fn get_game(&self) -> &Option<SupportedGame>;

  fn set_game(&self, game: Option<SupportedGame>) -> Box<dyn AppModel>;
}

#[derive(Serialize, Deserialize)]
pub struct AppModelSerenity {
  prefix: String,
  token: String,
  game: Option<SupportedGame>
}

impl AppModelSerenity {
  pub fn new(prefix: String, token: String, game: Option<SupportedGame>) -> AppModelSerenity {
    return AppModelSerenity {
      prefix: prefix,
      token: token,
      game: game
    }
  }

  #[allow(unreachable_patterns)]
  fn get_msg_parser(&self, game: &SupportedGame)
    -> Result<Box<dyn MessageParser + Send + Sync>, &str> {
    match game {
      SupportedGame::NDS => return Ok(Box::new(NDSUtils)),
      SupportedGame::FTL => return Ok(Box::new(FTLUtils)),
      _ => return Err("The given game is either invalid or unsupported.")
    }
  }

  #[allow(unreachable_patterns)]
  fn get_focus_checker(&self, game: &SupportedGame)
    -> Result<Box<dyn GameFocusChecker + Send + Sync>, &str> {
    match game {
      SupportedGame::NDS => return Ok(Box::new(NDSUtils)),
      SupportedGame::FTL => return Ok(Box::new(FTLUtils)),
      _ => return Err("The given game is either invalid or unsupported.")
    }
  }
}

impl Default for AppModelSerenity {
  fn default() -> AppModelSerenity {
    AppModelSerenity {
      prefix: ";".to_string(),
      token: "".to_string(),
      /*
      The config file wouldn't be created properly until I set this to
      something. Huh. It's hardly reasonable to already have this set to
      whatever, but better than nothing, I guess.
      */
      game: Some(SupportedGame::NDS)
    }
  }
}

#[async_trait]
impl AppModel for AppModelSerenity {

  async fn start(&self) -> Result<(), String> {
    match self.build_bot().await {
      Err(e) => return Err(e.to_string()),
      Ok(mut bot) => match bot.start().await {
        Err(e) => return Err(e),
        Ok(_) => return Ok(())
      }
    }
  }

  async fn build_bot(&self) -> Result<Box<dyn Bot + Send + Sync>, &str> {
    match self.game {
      None => return Err("Cannot build a bot without a supported game."),
      Some(game) => match self.get_msg_parser(&game) {
        Err(e) => return Err(e),
        Ok(msg_parser) => match self.get_focus_checker(&game) {
            Err(e) => return Err(e),
            Ok(focus_checker) => match self.token.to_string().as_str() {
              "" => return Err("Cannot build a bot without an OAuth token."),
              _ => {
                return Ok(Box::new(
                  BotSerenity::new(
                    self.prefix.to_string(),
                    self.token.to_string(),
                    msg_parser,
                    focus_checker)
                    .await
                ));
              }
            }
          }
      }
    }
  }

  fn get_prefix(&self) -> &String {
    return &self.prefix;
  }

  fn set_prefix(&self, prefix: String) -> Box<dyn AppModel> {
    return Box::new(
      AppModelSerenity {
        prefix: prefix,
        token: self.token.to_string(),
        game: self.game
    });
  }

  fn get_token(&self) -> &String {
    return &self.token;
  }

  fn set_token(&self, token: String) -> Box<dyn AppModel> {
    return Box::new(
      AppModelSerenity {
        prefix: self.prefix.to_string(),
        token: token,
        game: self.game
    });
  }

  fn get_game(&self) -> &Option<SupportedGame> {
    return &self.game;
  }

  fn set_game(&self, game: Option<SupportedGame>) -> Box<dyn AppModel> {
    return Box::new(
      AppModelSerenity {
        prefix: self.prefix.to_string(),
        token: self.token.to_string(),
        game: game
    });
  }
}