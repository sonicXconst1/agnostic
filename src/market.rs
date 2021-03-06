use super::currency;
use super::order;
use super::trading_pair::TradingPair;
use super::trading_pair::Coin;
use crate::trade::Trade;

pub type Future<Output> = std::pin::Pin<Box<dyn futures::Future<Output = Output>>>;

pub trait Accountant {
    fn ask(&self, coin: Coin) -> Future<Result<currency::Currency, String>>;
    fn ask_both(
        &self,
        first_coin: Coin,
        second_coin: Coin,
    ) -> Future<Result<(currency::Currency, currency::Currency), String>>;
    fn calculate_volume(&self, trading_pair: TradingPair, price: f64, amount: f64) -> f64;
    fn nearest_price(&self, trading_pair: TradingPair, price: f64) -> f64;
}

pub trait Trader {
    fn create_order(&self, order: order::Order) -> Future<Result<Trade, String>>;
    fn delete_order(&self, id: &str) -> Future<Result<(), String>>;
}

pub trait Sniffer {
    fn all_the_best_orders(
        &self,
        trading_pair: TradingPair,
        count: u32,
    ) -> Future<Result<Vec<order::Order>, String>>;
    fn get_my_orders(
        &self,
        trading_pair: TradingPair,
    ) -> Future<Result<Vec<order::OrderWithId>, String>>;
}
