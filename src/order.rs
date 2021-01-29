use super::coin;

#[derive(Clone, Debug)]
pub struct Order {
    pub coins: coin::CoinPair,
    pub price: f64,
    pub amount: f64,
}
