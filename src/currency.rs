use super::trading_pair::Coin;

#[derive(Clone, Debug)]
pub struct Currency {
    pub coin: Coin,
    pub amount: f64,
    pub held: f64,
}
