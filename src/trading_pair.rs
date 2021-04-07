#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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

#[derive(PartialEq, Eq, Clone, Debug)]
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

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
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

impl std::convert::TryFrom<(Coin, Coin)> for Coins {
    type Error = CoinsError;

    fn try_from(coins: (Coin, Coin)) -> Result<Coins, Self::Error> {
        match coins {
            (Coin::TON, Coin::USDT) => Ok(Coins::TonUsdt),
            other => Err(CoinsError::InvalidCoins(other)),
        }
    }
}

impl std::fmt::Display for Coins {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.base_coin(), self.quote_coin())
    }
}

pub enum CoinsError {
    InvalidInputString(String),
    InvalidCoins((Coin, Coin))
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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

const TON: &'static str = "ton";
const USDT: &'static str = "usdt";

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum Coin {
    TON,
    USDT,
}

impl std::fmt::Display for Coin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Coin::TON => write!(f, "{}", TON),
            Coin::USDT => write!(f, "{}", USDT),
        }
    }
}

impl std::convert::TryFrom<&str> for Coin {
    type Error = CoinsError;

    fn try_from(string: &str) -> Result<Coin, Self::Error> {
        match string {
            TON => Ok(Coin::TON),
            USDT => Ok(Coin::USDT),
            other => Err(CoinsError::InvalidInputString(other.to_owned())),
        }
    }
}

pub trait TradingPairConverter {
    type Pair: Clone;
    type Coin: std::hash::Hash + Eq + Clone;

    fn to_string(&self, trading_pair: TradingPair) -> String;
    fn to_pair(&self, trading_pair: TradingPair) -> Self::Pair;
    fn from_agnostic_coin(&self, coin: Coin) -> Self::Coin;
    fn to_agnostic_coin(&self, coin: Self::Coin) -> Option<Coin>;
}
