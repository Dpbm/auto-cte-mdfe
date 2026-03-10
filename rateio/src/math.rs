use crate::types::Price;

pub fn round_price(value:Price) -> Price{
    
    let increased = value*100.0;
    
    let frac = increased.fract();

    if frac >= 0.5{
        return increased.ceil()/100.0;
    }
    
    increased.floor()/100.0

}

#[cfg(test)]

mod tests{

    use super::*;

    #[test]
    fn test_round_for_fixing_first_value(){
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

    #[test]
    fn test_round_price_sum_rounding(){
        let total_cubicage = 12.9;
        let total_price = 3266.98;

        let first_price :Price = round_price(total_price * (2.16/total_cubicage));
        let second_price :Price = round_price(total_price * (0.12/total_cubicage));
        let total = first_price+second_price; 
        
        assert_eq!(total, 577.42004);
        assert_eq!(round_price(total), 577.42);
    }

    #[test]
    fn test_round_values_manually(){
        assert_eq!(round_price(130.45112345), 130.45);
        assert_eq!(round_price(130.45612345), 130.46);
    }
}
