use super::coin;

#[derive(Clone, Debug)]
pub struct Currency {
    pub coin: coin::Coin,
    pub amount: f64,
    pub held: f64,
}
