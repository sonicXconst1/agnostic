use super::coin;
use super::currency;
use super::order;

pub type Future<Output> = std::pin::Pin<Box<dyn futures::Future<Output = Output>>>;

pub trait Accountant {
    fn ask(&self, coin: coin::Coin) -> Future<Result<currency::Currency, String>>;
    fn ask_both(
        &self,
        coins: coin::CoinPair,
    ) -> Future<Result<(currency::Currency, currency::Currency), String>>;
    fn calculate_volume(&self, coins: coin::CoinPair, price: f64, amount: f64) -> f64;
}

pub trait Trader {
    fn create_order(&self, order: order::Order) -> Future<Result<(), String>>;
    fn update_order(&self, id: &str, new_order: order::Order) -> Future<Result<(), String>>;
    fn delete_order(&self, id: &str) -> Future<Result<(), String>>;
    fn create_trade_by_id(&self, order_id: &str) -> Future<Result<(), String>>;
    fn create_trade_from_order(&self, order: order::Order) -> Future<Result<(), String>>;
}

pub trait Sniffer {
    fn all_the_best_orders(
        &self,
        coins: coin::CoinPair,
        count: u32,
    ) -> Future<Result<Vec<order::Order>, String>>;
    fn the_best_order(&self, coins: coin::CoinPair) -> Future<Result<order::Order, String>>;
}
