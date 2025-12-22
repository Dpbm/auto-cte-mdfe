use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::math::round_price;

pub type TagName<'a> = &'a [u8];

pub type LoadNumber = u32;
pub type Quantity = u16;
pub type Cubicage = f32;
pub type Price = f32;
pub type Carrier = String;
pub type Client = String;
pub type LicensePlate = String;
pub type DANFE = String;

pub type Error = String;


#[derive(Debug,Clone,Default,Serialize,Deserialize)]
pub struct Packet{
    pub loads: Loads,
    pub errors: Vec<String>
}

// -------------------INTERMEDIATE OBJS-------------------------

#[derive(Debug,Clone)]
pub struct Data {
    pub danfe: DANFE,
    pub to: Client,
    pub by: Carrier,
    pub quantity: Quantity,
    pub load_number: LoadNumber,
    pub cubicage: Cubicage
}

// -------------------FOR LOADS---------------------------------

pub type Loads = HashMap<Carrier, Vec<Load>>;

#[derive(Debug,Clone,Default,Serialize,Deserialize)]
pub struct Load{
    pub deliveries: Vec<Delivery>,
    pub license_plate: LicensePlate,
    pub total_price: Price,
    pub total_cubicage: Cubicage,
    pub numer: LoadNumber
}

// -------------------FOR DELIVERY---------------------------------

#[derive(Debug,Clone,Default,Serialize,Deserialize)]
pub struct Delivery {
    pub danfe: Vec<DANFE>,
    pub to: Client,
    pub quantity: Quantity,
    pub price: Price,
    pub cubicage: Cubicage
}

// -------------------FOR EMAIL--------------------------------------

pub type EmailData = HashMap<LoadNumber,EmailLoadData>;

#[derive(Debug,Clone)]
pub struct EmailLoadData{
    pub price: Price,
    pub license_plate: LicensePlate
}


// -------------------IMPLEMENTATIONS---------------------------------
impl Load {
    pub fn update_load_delivery_data(&mut self){
        self.calculate_total_cubicage(); 
        self.calculate_price_for_each_delivery();

    }

    fn calculate_price_for_each_delivery(&mut self){
        if self.total_cubicage <= 0.0 { 
            return;
        }

        for delivery in &mut self.deliveries {
            delivery.price = round_price(self.total_price*(delivery.cubicage/self.total_cubicage));
        }
    }

    fn calculate_total_cubicage(&mut self) -> Cubicage {
        self.total_cubicage = self.deliveries
            .iter()
            .map(|delivery| delivery.cubicage)
            .reduce(|a,b| a + b)
            .expect("Failed on get total value");

        self.total_cubicage
    }
}


