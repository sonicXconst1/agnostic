use super::coin;

#[derive(Clone, Debug)]
pub struct Currency {
    pub coin: coin::Coin,
    pub amount: coin::Coin,
    pub held: coin::Coin,
}
