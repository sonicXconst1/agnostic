use super::trading_pair::TradingPair;

#[derive(Clone, Debug)]
pub struct Order {
    pub trading_pair: TradingPair,
    pub price: f64,
    pub amount: f64,
}

#[derive(Clone, Debug)]
pub struct OrderWithId {
    pub id: String,
    pub trading_pair: TradingPair,
    pub price: f64,
    pub amount: f64,
}

impl From<OrderWithId> for Order {
    fn from(order: OrderWithId) -> Order {
        Order {
            trading_pair: order.trading_pair,
            price: order.price,
            amount: order.amount,
        }
    }
}
