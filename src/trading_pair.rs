#[derive(Clone, Debug)]
pub struct TradingPair {
    pub coins: Coins,
    pub side: Side,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Coins {
    TonUsdt,
}

#[derive(Clone, Debug)]
pub enum Side {
    Sell,
    Buy
}

#[derive(PartialEq, Clone, Debug)]
pub enum Coin {
    TON,
    USDT,
    Unknown(String),
}

pub trait TradingPairConverter {
    type Pair: Clone;
    type Coin: PartialEq + Clone;

    fn to_string(&self, trading_pair: TradingPair) -> String;
    fn to_pair(&self, trading_pair: TradingPair) -> Self::Pair;
    fn from_agnostic_coin(&self, coin: Coin) -> Self::Coin;
    fn to_agnostic_coin(&self, coin: Self::Coin) -> Coin;
}
