use serde::{Deserialize, Serialize};

use crate::common::fetch_url;

#[derive(Serialize, Deserialize)]
pub struct GetAgent {
  data: Agent,
}

#[derive(Serialize, Deserialize)]
pub struct Agent {
  #[serde(alias = "accountId")]
  account_id: String,
  credits: u64,
  headquarters: String,
  #[serde(alias = "startingFaction")]
  starting_faction: String,
  symbol: String,
}

impl Agent {
  pub fn sector(&self) -> String {
    self
      .headquarters
      .split('-')
      .map(|s| s.to_owned())
      .next()
      .unwrap()
  }
  pub fn system(&self) -> String {
    let mut iter = self.headquarters.split('-').map(|s| s.to_owned());
    iter.next().unwrap() + "-" + &iter.next().unwrap()
  }
  pub fn waypoint(&self) -> String {
    self.headquarters.clone()
  }
}

pub async fn get_my_agent() -> Result<Agent, Box<dyn std::error::Error>> {
  Ok(
    fetch_url::<GetAgent>("https://api.spacetraders.io/v2/my/agent")
      .await?
      .data,
  )
}
