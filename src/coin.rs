#[derive(Clone, Debug)]
pub enum Coin {
    TON,
    USDT,
    BTC,
}

pub trait CoinConverter {
    fn to_string(&self, coin: Coin) -> String;
    fn to_coin<TCoin: Clone>(&self, coin: Coin) -> TCoin;
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
