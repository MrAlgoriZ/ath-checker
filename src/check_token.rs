use crate::config::config_types::Config;

use crate::client::BinanceClient;
use crate::types::TokenCaps;

use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use std::time::Duration;

pub async fn check(config: Config, only_priority_tokens: bool, must_return: bool) -> String {
    let client = BinanceClient::new().await;

    let prices = client.get_tokens_prices().await;
    let price_map: HashMap<String, f64> = prices.into_iter().map(|p| (p.symbol, p.price)).collect();

    let mut token_caps = TokenCaps::new();
    for symbol in price_map.keys() {
        token_caps.add(symbol);
    }

    let priority_tokens = config.priority_tokens;

    let mut processed = HashSet::new();

    let mut message = String::new();

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
                    "{} fallen of ATH {:.2}% (PRIORITY)",
                    symbol,
                    (ath - current_price) / ath * 100.0
                );
                if must_return {
                    writeln!(
                        &mut message,
                        "{} fallen of ATH {:.2}% (PRIORITY)",
                        symbol,
                        (ath - current_price) / ath * 100.0
                    )
                    .ok();
                }
            }

            processed.insert(symbol);
        }

        tokio::time::sleep(Duration::from_secs(
            config.token_check_interval_seconds as u64,
        ))
        .await;
    }

    if only_priority_tokens {
        return message;
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
                    "{} fallen of ATH {:.2}%",
                    token_cap.token,
                    (ath - current_price) / ath * 100.0
                );
                if must_return {
                    writeln!(
                        &mut message,
                        "{} fallen of ATH {:.2}%",
                        token_cap.token,
                        (ath - current_price) / ath * 100.0
                    )
                    .ok();
                }
            }
        }
        tokio::time::sleep(Duration::from_secs(
            config.token_check_interval_seconds as u64,
        ))
        .await;
    }

    message
}
