#[derive(Clone, Debug)]
pub enum Target {
    Market,
    Limit
}

impl std::fmt::Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Target::Market => "market",
            Target::Limit => "limit",
        })
    }
}

#[derive(Clone, Debug)]
pub struct TradingPair {
    pub coins: Coins,
    pub side: Side,
    pub target: Target,
}

impl TradingPair {
    pub fn reversed_side(self) -> Self {
        TradingPair {
            coins: self.coins,
            side: self.side.reversed(),
            target: self.target,
        }
    }

    pub fn coin_to_spend(&self) -> Coin {
        match self.side {
            Side::Sell => self.coins.base_coin(),
            Side::Buy => self.coins.quote_coin(),
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

impl std::fmt::Display for Coins {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.base_coin(), self.quote_coin())
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

impl std::fmt::Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Side::Buy => write!(f, "buy"),
            Side::Sell => write!(f, "sell"),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Coin {
    TON,
    USDT,
}

impl std::fmt::Display for Coin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Coin::TON => write!(f, "ton"),
            Coin::USDT => write!(f, "usdt"),
        }
    }
}

pub trait TradingPairConverter {
    type Pair: Clone;
    type Coin: PartialEq + Clone;

    fn to_string(&self, trading_pair: TradingPair) -> String;
    fn to_pair(&self, trading_pair: TradingPair) -> Self::Pair;
    fn from_agnostic_coin(&self, coin: Coin) -> Self::Coin;
    fn to_agnostic_coin(&self, coin: Self::Coin) -> Option<Coin>;
}
