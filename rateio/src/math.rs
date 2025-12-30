use crate::types::Price;

pub fn round_price(value:Price) -> Price{
    
    let increased = value*100.0;
    
    let frac = increased.fract();

    if frac >= 0.5{
        return increased.ceil() / 100.0;
    }

    return increased.floor() / 100.0;
}
