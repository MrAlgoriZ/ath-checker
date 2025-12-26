pub struct TokenCaps {
    pub tokens: Vec<TokenCap>,
}

impl TokenCaps {
    pub fn new() -> Self {
        TokenCaps { tokens: Vec::new() }
    }

    pub fn add(&mut self, token: &str) {
        let percent = if matches!(token, "BTCUSDT" | "ETHUSDT" | "BNBUSDT") {
            0.6
        } else if matches!(
            token,
            "ADAUSDT" | "SOLUSDT" | "DOTUSDT" | "ATOMUSDT" | "NEARUSDT" | "AVAXUSDT" | "MATICUSDT"
        ) {
            0.8
        } else {
            0.9
        };

        let token_cap = TokenCap {
            token: token.to_string(),
            percent,
        };
        self.add_token_cap(token_cap)
    }

    fn add_token_cap(&mut self, token_cap: TokenCap) {
        self.tokens.push(token_cap);
    }
}

pub struct TokenCap {
    pub token: String,
    percent: f64,
}

impl TokenCap {
    pub fn check_ath_fall(&self, ath: f64, current_price: f64) -> bool {
        if !ath.is_finite() || !current_price.is_finite() {
            return false;
        }

        if ath <= 0.0 || current_price <= 0.0 {
            return false;
        }

        if !(0.0 < self.percent && self.percent < 1.0) {
            return false;
        }

        if current_price >= ath {
            return false;
        }

        current_price <= ath * (1.0 - self.percent)
    }
}

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub struct ICandle {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

impl ICandle {
    pub fn new(open: f64, high: f64, low: f64, close: f64, volume: f64) -> Self {
        ICandle {
            open,
            high,
            low,
            close,
            volume,
        }
    }
}
