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
    fn to_string(&self, trading_pair: TradingPair) -> String;
}
