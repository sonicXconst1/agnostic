use super::currency;
use super::order;
use super::coin;

pub type Future<Output> = std::pin::Pin<Box<dyn futures::Future<Output = Output>>>;

pub trait Accountant {
    fn ask(&self, coin: coin::Coin) -> Future<Result<currency::Currency, String>>;
    fn ask_both(&self, coins: coin::CoinPair) -> Future<Result<(currency::Currency, currency::Currency), String>>;
    fn calculate_volume(
        &self,
        coins: coin::CoinPair,
        price: f64,
        volume: f64,
    ) -> f64;
}

pub trait Trader {
    fn create_order(&self, order: order::Order) -> bool;
    fn update_order(&self, id: &str, new_order: order::Order) -> bool;
    fn delete_order(&self, id: &str) -> bool;
    fn create_trade_by_id(&self, order_id: &str) -> bool;
    fn create_trade_from_order(&self, order: order::Order) -> bool;
}

pub trait Sniffer {
    fn all_the_best_orders(&self, coins: coin::CoinPair, count: u32) -> Vec<order::Order>;
    fn the_best_order(&self, coins: coin::CoinPair) -> Option<order::Order>;
}
