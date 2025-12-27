# ATH CHECKER - A simple Rust script for checking token drops from all-time-high price (ATH)

## Why is this needed
Filtering tokens by ATH drop allows you to display statistics on whether it's worth buying a certain token at the moment. Especially useful for cryptocurrency investors.

## Usage:
Run the **ath-checker.exe** file (https://github.com/MrAlgoriZ/ath-checker/releases/latest), or compile:
```bash
cargo build --release
```

After launching, a config.yaml file is initialized in the folder where the .exe file is located.

## Configuration
You can set **"priority_tokens"**, they are filled with the "-" sign. This field configures tokens that are checked first.

**"only_priority_tokens"** - a field that allows running checks only for the set **"priority_tokens"**.

**"filter_trash_tokens"** - whether to filter tokens by volatility. That is, check if it's not a scam token, and whether it's worth investing in (if the values are small, then the token is dead).

**"token_check_interval_seconds"** - additional delay between checks of each token (in seconds).

*// (Not implemented yet, but will be ready soon)*
**"alert_on_priority_token"** - allows notifying about drops in **"priority_tokens"** within a certain interval via Telegram. Parameters are configured: Enabled (**enabled**); **timeout_minutes** (interval in minutes); **chat_id** (your chat_id in Telegram); **telegram_token** (your bot token, can be obtained at https://t.me/BotFather)

## License and Dependencies
This project is under the **MIT** license.
For Binance requests, it uses the crate https://github.com/ccxt/binance-rs
README in Russian: https://github.com/MrAlgoriZ/ath-checker/README.ru.md 