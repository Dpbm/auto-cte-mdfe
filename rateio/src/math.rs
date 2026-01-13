use crate::types::Price;

pub fn round_price(value:Price) -> Price{
    
    let increased = value*100.0;
    
    let frac = increased.fract();

    if frac >= 0.5{
        return increased.ceil() / 100.0;
    }

    return increased.floor() / 100.0;
}

#[cfg(test)]

mod tests{

    use super::*;

    #[test]
    fn test_round(){
        let cubicages = vec![0.36, 0.84, 0.36, 1.08, 0.6, 7.68, 0.36, 0.6, 1.08];
        let total_price = 3723.43;

        let mut total_cubicage = 0.0;
        for cubicage in &cubicages{
            total_cubicage += cubicage;
        }

        let mut irregular_total = 0.0;
        let mut first_price = 0.0;

        for (index,cubicage) in cubicages.iter().enumerate(){
            let price = round_price(total_price*(cubicage/total_cubicage));
            if index == 0 {
                first_price = price;
            } 

            irregular_total += price;
        }

        let plain_fixed_value =  (total_price-irregular_total)+first_price;
        assert_ne!(103.42, plain_fixed_value);
        assert_eq!(103.42, round_price(plain_fixed_value));
    }
}
