use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub priority_tokens: Vec<String>,
    pub only_priority_tokens: bool,
    pub filter_trash_tokens: bool,
    pub token_check_interval_seconds: usize,
    pub alert_on_priority_tokens: AlertConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            priority_tokens: vec![
                "BTCUSDT".to_string(),
                "ETHUSDT".to_string(),
                "BNBUSDT".to_string(),
                "ADAUSDT".to_string(),
                "SOLUSDT".to_string(),
                "DOTUSDT".to_string(),
                "ATOMUSDT".to_string(),
                "NEARUSDT".to_string(),
                "AVAXUSDT".to_string(),
                "MATICUSDT".to_string(),
            ],
            only_priority_tokens: false,
            filter_trash_tokens: true,
            token_check_interval_seconds: 10,
            alert_on_priority_tokens: AlertConfig {
                enabled: false,
                timeout_minutes: 30,
                chat_id: "".to_string(),
                telegram_token: "".to_string()
            },
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AlertConfig {
    pub enabled: bool,
    pub timeout_minutes: usize,
    pub chat_id: String,
    pub telegram_token: String
}