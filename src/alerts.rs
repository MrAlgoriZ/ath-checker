use std::time::Duration;
use teloxide::prelude::*;

use crate::check_token::check;
use crate::config::config_types::Config;

pub struct TelegramHandler {
    pub config: Config,
}

impl TelegramHandler {
    pub fn new(config: Config) -> Self {
        TelegramHandler { config }
    }

    pub async fn run(&self) {
        let alert_cfg = &self.config.alert_on_priority_tokens;

        if !alert_cfg.enabled {
            eprintln!("Telegram alerts disabled in config");
            return;
        }

        let bot = Bot::new(alert_cfg.telegram_token.clone());

        let chat_id: i64 = alert_cfg.chat_id.parse().expect("Invalid telegram chat_id");

        let interval = Duration::from_secs(self.config.token_check_interval_seconds as u64);

        println!("\x1b[32m[INFO]\x1b[0m Telegram bot started");

        loop {
            let message = check(self.config.clone(), true, true).await;
            let text = format!("🚨 Alert:\n{}", message);

            if let Err(err) = bot.send_message(ChatId(chat_id), text).await {
                eprintln!("Failed to send telegram message: {:?}", err);
            }
            tokio::time::sleep(interval).await;
        }
    }
}
