use super::market;

pub trait Merchant
{
    type Accountant: market::Accountant + 'static;
    type Trader: market::Trader + 'static;
    type Sniffer: market::Sniffer + 'static;

    fn accountant(&self) -> Self::Accountant;
    fn trader(&self) -> Self::Trader;
    fn sniffer(&self) -> Self::Sniffer;
}
