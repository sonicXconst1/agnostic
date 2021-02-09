#[derive(Clone, Debug)]
pub struct TradingPair {
    pub coins: Coins,
    pub side: Side,
}

impl TradingPair {
    pub fn reversed(self) -> Self {
        TradingPair {
            coins: self.coins,
            side: self.side.reversed(),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Coins {
    TonUsdt,
}

impl Coins {
    pub fn base_coin(&self) -> Coin {
        match self {
            Coins::TonUsdt => Coin::TON,
        }
    }

    pub fn quote_coin(&self) -> Coin {
        match self {
            Coins::TonUsdt => Coin::USDT,
        }
    }
}

impl From<Coins> for (Coin, Coin) {
    fn from(coins: Coins) -> Self {
        match coins {
            Coins::TonUsdt => (Coin::TON, Coin::USDT),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Side {
    Sell,
    Buy
}

impl Side {
    pub fn reversed(self) -> Side {
        match self {
            Side::Sell => Side::Buy,
            Side::Buy => Side::Sell,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Coin {
    TON,
    USDT,
}

pub trait TradingPairConverter {
    type Pair: Clone;
    type Coin: PartialEq + Clone;

    fn to_string(&self, trading_pair: TradingPair) -> String;
    fn to_pair(&self, trading_pair: TradingPair) -> Self::Pair;
    fn from_agnostic_coin(&self, coin: Coin) -> Self::Coin;
    fn to_agnostic_coin(&self, coin: Self::Coin) -> Option<Coin>;
}
