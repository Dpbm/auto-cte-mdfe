use std::collections::HashMap;

use serde::{Deserialize, Serialize};


pub type TagName<'a> = &'a [u8];

pub type LoadNumber = u32;
pub type Cubicage = f32;
pub type Price = f32;
pub type Loads = HashMap<LoadNumber, Load>;
pub type EmailData = HashMap<LoadNumber,EmailLoadData>;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct EmailLoadData{
    pub load_number: LoadNumber,
    pub price:Price,
    pub license_plate:String
}


#[derive(Debug,Clone,Default,Serialize,Deserialize)]
pub struct Data {
    pub danfe: String,
    pub to: String,
    pub by: String,
    pub load_number: LoadNumber,
    pub cubicage: Cubicage,
    pub quantity: u16,
    pub price: Price
}

#[derive(Debug,Clone,Default,Serialize,Deserialize)]
pub struct Load{
    pub data: Vec<Data>,
    pub license_plate: String,
    pub total_price: Price,
    pub total_cubicage: Cubicage
}

#[derive(Debug,Clone,Default,Serialize,Deserialize)]
pub struct Packet{
    pub loads: Loads,
    pub email_data: EmailData,
    pub errors: Vec<String>
}

impl Data{
    pub fn calculate_shipping_price(&mut self, total_price:Price, total_cubicage:Cubicage){
        if total_cubicage > 0.0 {
            self.price = ((total_price*(self.cubicage/total_cubicage)) * 100.0).round() / 100.0;
        }
    }
}

impl Load{
    pub fn calculate_total_cubicage(&mut self) -> Cubicage {
        self.total_cubicage = self.data
            .iter()
            .map(|value| value.cubicage)
            .reduce(|a,b| a + b)
            .expect("Failed on get total value");

        self.total_cubicage
    }

}

