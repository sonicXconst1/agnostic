use super::coin;

#[derive(Clone, Debug)]
pub struct Order {
    pub coins: coin::CoinPair,
    pub price: f64,
    pub amount: f64,
}

#[derive(Clone, Debug)]
pub struct OrderWithId {
    pub id: String,
    pub coins: coin::CoinPair,
    pub price: f64,
    pub amount: f64,
}
