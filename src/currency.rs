use crate::trading_pair::Coin;

#[derive(Clone, Debug)]
pub struct Currency {
    pub coin: Coin,
    pub held: f64,
    pub amount: f64,
}
