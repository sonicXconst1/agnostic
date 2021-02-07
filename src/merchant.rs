use super::market;

pub trait Merchant
{
    type Accountant: market::Accountant + 'static;
    type Trader: market::Trader + 'static;
    type Sniffer: market::Sniffer + 'static;

    fn accountant(&self) -> std::sync::Arc<dyn market::Accountant>;
    fn trader(&self) -> std::sync::Arc<dyn market::Trader>;
    fn sniffer(&self) -> std::sync::Arc<dyn market::Sniffer>;
}
