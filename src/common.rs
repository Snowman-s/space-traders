use reqwest::Client;
use serde::Deserialize;
use std::env;

pub async fn fetch_url<T: for<'a> Deserialize<'a>>(
  url: &str,
) -> Result<Box<T>, Box<dyn std::error::Error>> {
  let token = env::var("TOKEN").unwrap();
  println!("url = {}", url);

  let responce = Client::new()
    .get(url)
    .bearer_auth(&token)
    .send()
    .await?
    .text()
    .await?;
  let json: T = serde_json::from_str(&responce)?;

  Ok(Box::new(json))
}

pub async fn post_url<T: for<'a> Deserialize<'a>>(
  url: &str,
) -> Result<Box<T>, Box<dyn std::error::Error>> {
  let token = env::var("TOKEN").unwrap();
  println!("url = {}", url);

  let responce = Client::new()
    .post(url)
    .header("Content-Length", 0)
    .bearer_auth(&token)
    .send()
    .await?
    .text()
    .await?;
  let json: T = serde_json::from_str(&responce)?;

  Ok(Box::new(json))
}
