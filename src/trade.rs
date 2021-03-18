use crate::trading_pair::TradingPair;
use crate::order::OrderWithId;

#[derive(Clone, Debug)]
pub enum Trade {
    Market(TradeResult),
    Limit(OrderWithId),
}

#[derive(Clone, Debug)]
pub struct TradeResult {
    pub id: String,
    pub trading_pair: TradingPair,
    pub price: f64,
    pub amount: f64,
}
