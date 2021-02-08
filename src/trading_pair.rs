#[derive(Clone, Debug)]
pub struct TradingPair {
    pub coins: Coins,
    pub side: Side,
}

#[derive(Clone, Debug)]
pub enum Coins {
    TonUsdt,
}

#[derive(Clone, Debug)]
pub enum Side {
    Sell,
    Buy
}

pub trait TradingPairConverter {
    type Pair: Clone;

    fn to_string(&self, trading_pair: TradingPair) -> String;
    fn to_pair(&self, trading_pair: TradingPair) -> Self::Pair;
}
