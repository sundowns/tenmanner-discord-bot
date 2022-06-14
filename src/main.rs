extern crate dotenv;

use dotenv::dotenv;
use std::env;

#[derive(Debug)]
struct AppConfig {
  discord_token: String,
}

fn main() {
  let config: AppConfig = load_config();
  println!("{:?}", config.discord_token);
}

fn load_config() -> AppConfig {
  dotenv().ok();
  let discord_token = env::var("DISCORD_TOKEN").unwrap();
  AppConfig { discord_token }
}
