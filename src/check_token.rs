use crate::client::BinanceClient;
use crate::config::config::load_config;
use crate::types::TokenCaps;

use std::collections::{HashMap, HashSet};

pub async fn check() {
    let client = BinanceClient::new().await;
    let config: crate::config::config_types::Config = load_config("config.yaml");

    let prices = client.get_tokens_prices().await;
    let price_map: HashMap<String, f64> = prices.into_iter().map(|p| (p.symbol, p.price)).collect();

    let mut token_caps = TokenCaps::new();
    for symbol in price_map.keys() {
        token_caps.add(symbol);
    }

    let priority_tokens = config.priority_tokens;

    let mut processed = HashSet::new();

    for symbol in priority_tokens {
        if let Some(current_price) = price_map.get(&symbol) {
            let ath = client.get_ath(&symbol).await;

            let token_cap = token_caps
                .tokens
                .iter()
                .find(|t| t.token == symbol)
                .unwrap();

            if token_cap.check_ath_fall(ath, *current_price) {
                println!(
                    "{} упал от ATH {:.2}% (ПРИОРИТЕТ)",
                    symbol,
                    (ath - current_price) / ath * 100.0
                );
            }

            processed.insert(symbol);
        }
    }

    if config.only_priority_tokens {
        return;
    }

    for token_cap in &token_caps.tokens {
        if processed.contains(token_cap.token.as_str()) {
            continue;
        }

        if let Some(current_price) = price_map.get(&token_cap.token) {
            let ath = client.get_ath(&token_cap.token).await;

            if token_cap.check_ath_fall(ath, *current_price)
                && (!config.filter_trash_tokens
                    || (client.get_volatility(&token_cap.token, "1d", 31).await) > 0.03)
            {
                println!(
                    "{} упал от ATH {:.2}%",
                    token_cap.token,
                    (ath - current_price) / ath * 100.0
                );
            }
        }
    }
}
