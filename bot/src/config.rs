use dotenv::{dotenv, from_filename};
use std::env;

#[derive(Debug)]
pub struct AppConfig {
    pub discord_token: String,
    pub guild_id: u64,
    pub lobby_channel_id: u64,
    pub organiser_role_id: u64,
    pub google_sheets_client_id: String,
    pub google_sheets_client_secret: String,
}

pub fn load_config() -> AppConfig {
    let environment = env::var("ENVIRONMENT").unwrap_or("default".to_string());
    match environment.as_str() {
        "prod" => from_filename(".env.prod").ok(),
        "dev" => from_filename(".env.dev").ok(),
        _ => dotenv().ok(),
    };
    let discord_token = env::var("DISCORD_TOKEN").unwrap();
    let guild_id = env::var("GUILD_ID")
        .expect("Expected GUILD_ID in .env")
        .parse()
        .expect("GUILD_ID must be an integer");
    let lobby_channel_id = env::var("LOBBY_CHANNEL_ID")
        .expect("Expected LOBBY_CHANNEL_ID in .env")
        .parse()
        .expect("LOBBY_CHANNEL_ID must be an integer");
    let organiser_role_id = env::var("ORGANISER_ROLE_ID")
        .expect("Expected ORGANISER_ROLE_ID in .env")
        .parse()
        .expect("ORGANISER_ROLE_ID must be an integer");
    let google_sheets_client_id = env::var("GOOGLE_SHEETS_CLIENT_ID").unwrap();
    let google_sheets_client_secret = env::var("GOOGLE_SHEETS_CLIENT_SECRET").unwrap();
    AppConfig {
        discord_token,
        guild_id,
        lobby_channel_id,
        organiser_role_id,
        google_sheets_client_id,
        google_sheets_client_secret,
    }
}
