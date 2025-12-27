use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub priority_tokens: Vec<String>,
    pub only_priority_tokens: bool,
    pub filter_trash_tokens: bool,
    pub token_check_interval_seconds: usize,
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
        }
    }
}
