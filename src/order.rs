use super::trading_pair::TradingPair;

#[derive(Clone, Debug)]
pub struct Order {
    pub trading_pair: TradingPair,
    pub price: f64,
    pub amount: f64,
}

#[derive(Clone, Debug)]
pub struct OrderWithId {
    pub id: String,
    pub trading_pair: TradingPair,
    pub price: f64,
    pub amount: f64,
}
