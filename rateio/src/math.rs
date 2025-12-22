use crate::types::Price;

pub fn round_price(value:Price) -> Price{
    (value * 100.0).round() / 100.0
}
