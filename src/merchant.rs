use super::market;

pub trait Merchant
{
    fn id() -> &'static str;

    fn accountant(&self) -> std::sync::Arc<dyn market::Accountant>;
    fn trader(&self) -> std::sync::Arc<dyn market::Trader>;
    fn sniffer(&self) -> std::sync::Arc<dyn market::Sniffer>;
}
