use super::market;

pub trait Merchant
{
    type Accountant: market::Accountant;
    type Trader: market::Trader;
    type Sniffer: market::Sniffer;

    fn accountant(&self) -> &Self::Accountant;
    fn trader(&self) -> &Self::Trader;
    fn sniffer(&self) -> &Self::Sniffer;
}
