use crate::trading_pair::TradingPair;
use crate::order::OrderWithId;

#[derive(PartialEq, Clone, Debug)]
pub enum Trade {
    Market(TradeResult),
    Limit(OrderWithId),
}

impl Trade {
    pub fn trading_pair(&self) -> TradingPair {
        match self {
            Trade::Market(result) => result.trading_pair.clone(),
            Trade::Limit(result) => result.trading_pair.clone(),
        }
    }

    pub fn price(&self) -> f64 {
        match self {
            Trade::Market(result) => result.price,
            Trade::Limit(result) => result.price,
        }
    }

    pub fn amount(&self) -> f64 {
        match self {
            Trade::Market(result) => result.amount,
            Trade::Limit(result) => result.amount,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct TradeResult {
    pub id: String,
    pub trading_pair: TradingPair,
    pub price: f64,
    pub amount: f64,
}
