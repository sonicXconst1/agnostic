use super::trading_pair::TradingPair;

#[derive(Clone, Debug)]
pub struct Currency {
    pub coin: TradingPair,
    pub amount: f64,
    pub held: f64,
}
