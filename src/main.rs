use crate::config::config::{ensure_config_exists, load_config};

mod alerts;
mod check_token;
mod client;
mod config;
mod types;

#[tokio::main]
async fn main() {
    ensure_config_exists("config.yaml");

    let config = load_config("config.yaml");
    if config.alert_on_priority_tokens.enabled {
        alerts::TelegramHandler::new(config.clone()).run().await;
    } else {
        check_token::check(config.clone(), config.only_priority_tokens, false).await;
    }
}
