use crate::model::key::KeyInputtable;
use serenity::{
  async_trait,
  client::{
    Client,
    Context,
    EventHandler
  },
  framework::standard::StandardFramework,
  model::{
    channel::Message,
    id::GuildId,
    gateway::{
      Ready,
      Activity
    }
  }
};
use std::{
  sync::{
    Arc,
    atomic::{
      AtomicBool,
      Ordering
    }
  },
  time::Duration
};
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use typemap_rev::{TypeMap, TypeMapKey};

#[async_trait]
pub trait Bot {
  async fn start(&mut self) -> Result<(), String>;
}

pub trait MessageParser {
  fn parse_msg(&self, content: &str) -> Result<Box<dyn KeyInputtable + Send + Sync>, ()>;
}

pub trait GameFocusChecker {
  fn game_focused(&self) -> bool;
}

struct InputStack;

impl TypeMapKey for InputStack {
  type Value = Arc<RwLock<Vec<Box<dyn KeyInputtable + Send + Sync>>>>;
}

struct FocusChecker;

impl TypeMapKey for FocusChecker {
  type Value = Arc<RwLock<Box<dyn GameFocusChecker + Send + Sync>>>;
}

// Convenience function for retrieving the Arc wrapper around the input stack.
// This will allow us to safely write to the stack.
async fn arc_input_stack(ctx: &Context)
  -> Arc<RwLock<Vec<Box<dyn KeyInputtable + Send + Sync>>>> {
  let data_read: RwLockReadGuard<'_, TypeMap> = ctx.data.read().await;
  return data_read.get::<InputStack>().expect("Expected an InputStack in TypeMap.").clone();
}

async fn arc_focus_checker(ctx: &Context)
  -> Arc<RwLock<Box<dyn GameFocusChecker + Send + Sync>>> {
  let data_read: RwLockReadGuard<'_, TypeMap> = ctx.data.read().await;
  return data_read.get::<FocusChecker>().expect("Expected a FocusChecker in TypeMap.").clone(); 
}

// A function representing a command for a generic input.
// In general, the idea is that it'll pass the message content to the parser,
// after which it's supposed to retrieve an input to add to the stack.
async fn push_input(msg_parser: &Box<dyn MessageParser + Send + Sync>, ctx: &Context, content: &str) -> () {
  let input_stack_lock = arc_input_stack(ctx).await;

  // This enables us to write to the input stack without deadlocking the bot.
  // The only problem is that this is obnoxiously ugly.
  let mut input_stack:
    RwLockWriteGuard<Vec<Box<dyn KeyInputtable + Send + Sync>>> =
    input_stack_lock.write().await;
  match msg_parser.parse_msg(content) {
    Err(_) => (),
    Ok(input) => input_stack.insert(0, input)
  }
}

async fn pop_input(ctx: Arc<Context>) -> () {
  let input_stack_lock = arc_input_stack(&ctx).await;
  let focus_checker_lock = arc_focus_checker(&ctx).await;

  let mut input_stack:
    RwLockWriteGuard<Vec<Box<dyn KeyInputtable + Send + Sync>>> =
    input_stack_lock.write().await;
  // println!("Size of input stack: {}", input_stack.len());
  let focus_checker = focus_checker_lock.write().await;
  
  // I enjoy pattern matching in this language though.
  if focus_checker.game_focused() {
    match input_stack.pop() {
      None => return,
      Some(input) => {
        if input.get_presses() <= &20 {
          match input.pop() {
            None => return,
            Some(next) => input_stack.push(next)
          }
        }
      }
    }
  }
  /*
  match input_stack.pop() {
    None => return,
    Some(input) => {
      if input.get_presses() <= &10 {
        if focus_checker.game_focused() {
          match input.pop() {
            None => return,
            Some(next) => input_stack.insert(0, next)
          }
        }
        /*
        loop {
          if focus_checker.game_focused() {
            match input.pop() {
              None => break,
              Some(next) => input = next
            }
          }
        }
        */
      }
    }
  }
  */
}

struct Handler {
  prefix: String,
  msg_parser: Box<dyn MessageParser + Send + Sync>,
  input_loop_running: AtomicBool
}

#[async_trait]
impl EventHandler for Handler {
  async fn message(&self, ctx: Context, msg: Message) -> () {
    if &msg.content.len() > &1
      && &msg.content.chars().nth(0).unwrap().to_string() == &self.prefix {
      let slice: &str = &msg.content[1..];
      push_input(&self.msg_parser, &ctx, &slice).await;
    }
  }

  async fn ready(&self, ctx: Context, _ready: Ready) {
    let ctx = Arc::new(ctx);
    ctx.set_activity(
      Activity::listening(
        format!("the prefix \"{}\"", self.prefix)
      )
    ).await;
  }

  // I don't totally know what this does, but it supposedly allows you to run
  // another loop at the same time as the bot's current thread.
  async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
    let ctx = Arc::new(ctx);
    if !self.input_loop_running.load(Ordering::Relaxed) {
      let ctx_clone = Arc::clone(&ctx);

      tokio::spawn(async move {
        loop {
          pop_input(Arc::clone(&ctx_clone)).await;
          tokio::time::sleep(Duration::from_millis(1)).await;
        }
      });
    }
  }
}

pub struct BotSerenity {
  client: Client
}

impl BotSerenity {

  pub async fn new(prefix: String, token: String, msg_parser: Box<dyn MessageParser + Send + Sync>, focus_checker: Box<dyn GameFocusChecker + Send + Sync>) -> BotSerenity {
    let framework = StandardFramework::new()
      .configure(|c| c
        .with_whitespace(true));

    let mut bot: BotSerenity = BotSerenity {
      client: Client::builder(token)
        .event_handler(Handler {
          prefix: prefix.to_string(),
          msg_parser: msg_parser,
          input_loop_running: AtomicBool::new(false)
        })
        .framework(framework)
        .await
        .expect("Error creating client.")
    };
  
    {
      let mut data = (&mut bot).client.data.write().await;
      data.insert::<InputStack>(Arc::new(RwLock::new(Vec::new())));
      data.insert::<FocusChecker>(Arc::new(RwLock::new(focus_checker)));
    }

    return bot;
  }
}

#[async_trait]
impl Bot for BotSerenity {
  async fn start(&mut self) -> Result<(), String> {
    match self.client.start().await {
      Err(e) => return Err(e.to_string()),
      Ok(_) => return Ok(())
    }
  }
}