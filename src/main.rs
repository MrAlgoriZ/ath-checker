mod alerts;
mod check_token;
mod client;
mod types;

#[tokio::main]
async fn main() {
    check_token::check().await
}
