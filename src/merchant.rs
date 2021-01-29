use super::balance;

pub trait Merchant
{
    type Accountant: balance::Accountant;
    type Trader: balance::Trader;
    type Sniffer: balance::Sniffer;

    fn accountant(&self) -> &Self::Accountant;
    fn trader(&self) -> &Self::Trader;
    fn sniffer(&self) -> &Self::Sniffer;
}
