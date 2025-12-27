use binance::model::SymbolPrice;
use std::sync::Arc;
use tokio::task;

use binance::api::Binance;
use binance::market::Market;

use crate::types::ICandle;

pub struct BinanceClient {
    market: Arc<Market>,
}

impl BinanceClient {
    pub async fn new() -> Self {
        let market = tokio::task::spawn_blocking(|| Binance::new(None, None))
            .await
            .expect("spawn_blocking failed");

        BinanceClient {
            market: Arc::new(market),
        }
    }

    async fn run_blocking<F, T>(&self, f: F, default: T) -> T
    where
        F: FnOnce(&Market) -> Result<T, String> + Send + 'static,
        T: Send + 'static,
    {
        let market = Arc::clone(&self.market);
        let handle = task::spawn_blocking(move || f(&market));

        match handle.await {
            Ok(Ok(result)) => result,
            Ok(Err(e)) => {
                eprintln!("BinanceClient error: {}", e);
                default
            }
            Err(join_err) => {
                eprintln!("JoinError: {}", join_err);
                default
            }
        }
    }

    pub async fn get_tokens_prices(&self) -> Vec<SymbolPrice> {
        return self
            .run_blocking(
                move |market| match market.get_all_prices() {
                    Ok(binance::model::Prices::AllPrices(prices)) => Ok(prices),
                    Err(e) => Err(format!("Binance error {:?}", e)),
                },
                Vec::new(),
            )
            .await;
    }

    pub async fn get_ath(&self, symbol: &str) -> f64 {
        let klines = self.fetch_ohlcv(symbol, "1M", 12).await;

        let ath = klines.iter().map(|k| k.high).fold(f64::MIN, f64::max);
        ath
    }

    pub async fn fetch_ohlcv(&self, token: &str, timeframe: &str, limit: usize) -> Vec<ICandle> {
        let token = token.to_string();
        let timeframe = timeframe.to_string();

        return self
            .run_blocking(
                move |market| {
                    let mut ohlcv_list: Vec<ICandle> = Vec::new();

                    match market.get_klines(&token, &timeframe, limit as u16, None, None) {
                        Ok(binance::model::KlineSummaries::AllKlineSummaries(klines)) => {
                            for kline in klines {
                                let open: f64 = kline.open.parse().unwrap_or(f64::MIN);
                                let high: f64 = kline.high.parse().unwrap_or(f64::MIN);
                                let low: f64 = kline.low.parse().unwrap_or(f64::MIN);
                                let close: f64 = kline.close.parse().unwrap_or(f64::MIN);
                                let volume: f64 = kline.volume.parse().unwrap_or(f64::MIN);

                                ohlcv_list.push(ICandle::new(open, high, low, close, volume));
                            }
                            Ok(ohlcv_list)
                        }
                        Err(e) => Err(format!("Binance error: {:?}", e)),
                    }
                },
                Vec::new(),
            )
            .await;
    }

    pub async fn get_volatility(&self, symbol: &str, timeframe: &str, limit: usize) -> f64 {
        let klines = self.fetch_ohlcv(symbol, timeframe, limit).await;

        if klines.is_empty() {
            return 0.0;
        }

        let mut volatilities = Vec::with_capacity(klines.len());

        for candle in klines.iter() {
            let high = candle.high;
            let low = candle.low;
            let open = candle.open;
            let volatility = (high - low) / open;
            volatilities.push(volatility);
        }

        let sum = volatilities.iter().sum::<f64>();
        let avg = sum / volatilities.len() as f64;

        avg
    }
}
