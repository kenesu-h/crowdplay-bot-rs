pub mod model;
pub mod game;

use crate::model::app_model::{AppModel, AppModelSerenity};
use confy::ConfyError;

fn load_config() -> Result<Box<dyn AppModel>, ConfyError> {
  let cfg: AppModelSerenity = confy::load_path("./config.toml")?;
  return Ok(Box::new(cfg));
}

#[tokio::main]
async fn main() -> Result<(), String> {
  let cfg: Box<dyn AppModel> = load_config()
    .expect("Expected a model to be generated from a config.");
 
  // The next step aside from the config is probably to start the client in
  // another thread. I'm not looking forward to that though.
  match cfg.start().await {
    Err(e) => return Err(e),
    Ok(_) => return Ok(())
  }
}