pub mod common;
pub mod my;

use dotenv::dotenv;
use serde_json::Value;
use std::{borrow::Borrow, env, future::Future, io::Write};

use crate::{
  common::{fetch_url, post_url},
  my::stat::get_my_agent,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  dotenv().unwrap();

  let _token = env::var("TOKEN").unwrap();

  println!("Welcome to *fanmade* SpaceTrader (v2) CUI!");

  loop {
    print!("$ ");
    std::io::stdout().flush()?;
    let mut line = "".to_owned();
    std::io::stdin().read_line(&mut line)?;
    let line = line.trim();

    let splited: Vec<&str> = line.split_whitespace().collect();

    match accept_command(splited).await {
      Ok(CommandResult::Nothing) => {}
      Ok(CommandResult::Exit) => return Ok(()),
      Err(err) => println!("error: {}", err),
    }
  }
}

enum CommandResult {
  Nothing,
  Exit,
}

async fn accept_command(cmd: Vec<&str>) -> Result<CommandResult, Box<dyn std::error::Error>> {
  match cmd[0] {
    "exit" => return Ok(CommandResult::Exit),
    "stat" => {
      println!("{}", serde_json::to_string_pretty(&get_my_agent().await?)?);
    }
    "here" => {
      let agent = get_my_agent().await?;
      let url = format!(
        "https://api.spacetraders.io/v2/systems/{}/waypoints/{}",
        agent.system(),
        agent.waypoint()
      );
      let value: Box<Value> = fetch_url(&url).await?;
      println!("{}", serde_json::to_string_pretty(&value)?);
    }
    "cont" => match cmd.get(1) {
      None => {
        let cont: Box<Value> = fetch_url("https://api.spacetraders.io/v2/my/contracts").await?;
        println!("{}", serde_json::to_string_pretty(&cont)?);
      }
      Some(&"accept") => {
        let url = format!(
          "https://api.spacetraders.io/v2/my/contracts/{}/accept",
          cmd[2]
        );
        let value: Box<Value> = post_url(&url).await?;
        println!("{}", serde_json::to_string_pretty(&value)?);
      }
      Some(_) => {
        println!("Unknown Command");
      }
    },
    "sys" => match cmd.get(1) {
      None => {
        let cont: Box<Value> = fetch_url("https://api.spacetraders.io/v2/systems").await?;
        println!("{}", serde_json::to_string_pretty(&cont)?);
      }
      Some(&"search") => {
        let url = format!(
          "https://api.spacetraders.io/v2/systems/{}/waypoints?traits={}",
          cmd[2], cmd[3]
        );
        let value: Box<Value> = fetch_url(&url).await?;
        println!("{}", serde_json::to_string_pretty(&value)?);
      }
      Some(_) => {
        println!("Unknown Command");
      }
    },
    _ => {
      println!("Unknown Command");
    }
  };

  Ok(CommandResult::Nothing)
}
