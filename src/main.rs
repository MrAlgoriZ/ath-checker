mod check_token;
mod client;
mod config;
mod types;

#[tokio::main]
async fn main() {
    config::config::ensure_config_exists("config.yaml");
    check_token::check().await
}
