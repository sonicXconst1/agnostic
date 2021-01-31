#[derive(Clone, Debug)]
pub enum Coin {
    TON,
    USDT,
    BTC,
    Unknown(String),
}

pub trait CoinConverter {
    type Coin: Clone;

    fn to_string(&self, coin: Coin) -> String;
    fn to_coin(&self, coin: Coin) -> Self::Coin;
    fn from_coin(&self, coin: Self::Coin) -> Coin;
}

#[derive(Clone, Debug)]
pub struct CoinPair {
    pub sell: Coin,
    pub buy: Coin,
}

impl CoinPair {
    pub fn new(sell: Coin, buy: Coin) -> Self {
        Self {
            sell,
            buy
        }
    }

    pub fn reversed(self) -> Self {
        Self {
            sell: self.buy,
            buy: self.sell,
        }
    }

    pub fn with_delimiter<Converter: CoinConverter>(
        self,
        converter: &Converter,
        delimiter: char
    ) -> String {
        format!(
            "{}{}{}",
            &converter.to_string(self.sell),
            delimiter,
            &converter.to_string(self.buy))
    }
}
