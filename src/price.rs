use crate::trading_pair::Target;
use crate::trading_pair::Side;

#[derive(Copy, Clone, Debug)]
pub struct Price {
    value: f64
}

impl From<f64> for Price {
    fn from(value: f64) -> Price {
        Price { value }
    }
}

impl Price {
    pub fn direct(&self) -> f64 {
        self.value
    }

    pub fn reversed(&self) -> f64 {
        1.0 / self.value
    }
}

pub fn convert_to_base_coin_price(target: Target, side: Side, price: &Price) -> f64 {
    match (target, side) {
        (Target::Market, Side::Buy) => price.direct(),
        (Target::Market, Side::Sell) => price.reversed(),
        (Target::Limit, Side::Buy) => price.reversed(),
        (Target::Limit, Side::Sell) => price.direct(),
    }
}

pub fn convert_to_base_coin_amount(
    target: Target,
    side: Side,
    price: &Price,
    amount: f64) -> f64 {
    match (target, side) {
        (Target::Market, Side::Buy) => amount,
        (Target::Market, Side::Sell) => price.direct() * amount,
        (Target::Limit, Side::Buy) => price.direct() * amount,
        (Target::Limit, Side::Sell) => amount,
    }
}

pub fn currency_to_coin_amount(
    currency: &crate::currency::Currency,
    coins: &crate::trading_pair::Coins,
    price: &Price,
) -> Option<f64> {
    if currency.coin == coins.base_coin() {
        Some(currency.amount)
    } else if currency.coin == coins.quote_coin() {
        Some(currency.amount * price.reversed())
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use crate::trading_pair::Target;
    use crate::trading_pair::Side;

    #[test]
    fn price() {
        let initial_price = 2.0;
        let price: super::Price = initial_price.into();
        assert_eq!(initial_price, price.direct());
        assert_eq!(1.0 / initial_price, price.reversed());
    }

    #[test]
    fn convert_to_base_coin_price() {
        let target = Target::Market;
        let side = Side::Buy;
        let price = 2.0.into();
        let calculated_price = super::convert_to_base_coin_price(target, side, &price);
        assert_eq!(price.direct(), calculated_price);
        let target = Target::Market;
        let side = Side::Sell;
        let calculated_price = super::convert_to_base_coin_price(target, side, &price);
        assert_eq!(price.reversed(), calculated_price);
        let target = Target::Limit;
        let side = Side::Buy;
        let calculated_price = super::convert_to_base_coin_price(target, side, &price);
        assert_eq!(price.reversed(), calculated_price);
        let target = Target::Limit;
        let side = Side::Sell;
        let calculated_price = super::convert_to_base_coin_price(target, side, &price);
        assert_eq!(price.direct(), calculated_price);
    }

    #[test]
    fn convert_to_base_coin_amount() {
        let target = Target::Market;
        let side = Side::Buy;
        let price = 2.0.into();
        let amount = 10.0;
        let calculated_amount = super::convert_to_base_coin_amount(target, side, &price, amount);
        assert_eq!(amount, calculated_amount);
        let target = Target::Market;
        let side = Side::Sell;
        let calculated_amount = super::convert_to_base_coin_amount(target, side, &price, amount);
        assert_eq!(amount * price.direct(), calculated_amount);
        let target = Target::Limit;
        let side = Side::Buy;
        let calculated_amount = super::convert_to_base_coin_amount(target, side, &price, amount);
        assert_eq!(amount * price.direct(), calculated_amount);
        let target = Target::Limit;
        let side = Side::Sell;
        let calculated_amount = super::convert_to_base_coin_amount(target, side, &price, amount);
        assert_eq!(amount, calculated_amount);
    }

    #[test]
    fn currency_to_base_coin_amount() {
        use crate::trading_pair::Coins;
        use crate::currency::Currency;
        let coins = Coins::TonUsdt;
        let initial_amount = 2.0;
        let currency = Currency {
            coin: coins.base_coin(),
            held: 0.0,
            amount: initial_amount,
        };
        let order_price = 2.0.into();
        let calculated_amount = super::currency_to_coin_amount(
            &currency,
            &coins,
            &order_price);
        assert_eq!(
            calculated_amount.expect("Invalid coins"), 
            initial_amount,
            "Failed to calculate base coin amount"); 
        let currency = Currency {
            coin: coins.quote_coin(),
            held: 0.0,
            amount: initial_amount,
        };
        let calculated_amount = super::currency_to_coin_amount(
            &currency,
            &coins,
            &order_price);
        assert_eq!(
            calculated_amount.expect("Invalid coins"),
            initial_amount * order_price.reversed(),
            "Falied to calculated base coin amount for quote coin")
    }
}
