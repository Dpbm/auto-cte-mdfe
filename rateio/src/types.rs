use std::collections::HashMap;
use std::num::{ParseIntError, ParseFloatError};
use std::fmt;

use quick_xml::errors::Error as quick_xml_ERROR;
use quick_xml::encoding::EncodingError;

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
pub type Key = String;

pub type Error = String;

#[derive(Debug)]
pub enum ParseErrors{
   ParseInt(ParseIntError),
   ParseFloat(ParseFloatError),
   XMLError(quick_xml_ERROR),
   EncodingXMLError(EncodingError)
}

impl fmt::Display for ParseErrors{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            ParseErrors::ParseInt(int_error) => write!(f,"Couldn't parse int value: {}", int_error),
            ParseErrors::ParseFloat(float_error) => write!(f,"Couldn't parse float value: {}", float_error),
            ParseErrors::XMLError(xml_error) => write!(f,"Couldn't parse xml: {}", xml_error),
            ParseErrors::EncodingXMLError(encoding_error) => write!(f,"Failed on Decode: {}", encoding_error)
        }
    }
}

impl From<ParseIntError> for ParseErrors {
    fn from(e: ParseIntError) -> Self {
        ParseErrors::ParseInt(e)
    }
}

impl From<ParseFloatError> for ParseErrors {
    fn from(e: ParseFloatError) -> Self {
        ParseErrors::ParseFloat(e)
    }
}

impl From<quick_xml_ERROR> for ParseErrors {
    fn from(e: quick_xml_ERROR) -> Self {
        ParseErrors::XMLError(e)
    }
}

impl From<EncodingError> for ParseErrors {
    fn from(e: EncodingError) -> Self {
        ParseErrors::EncodingXMLError(e)
    }
}

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
    pub cubicage: Cubicage,
    pub key: Key
}

pub type MultipleData = HashMap<LoadNumber, Vec<Data>>;

// -------------------FOR LOADS---------------------------------

pub type LoadsByNumberData = HashMap<LoadNumber, Load>;
pub type Loads = HashMap<Carrier, LoadsByNumberData>;

#[derive(Debug,Clone,Default,Serialize,Deserialize)]
pub struct Load{
    pub deliveries: Vec<Delivery>,
    pub license_plate: LicensePlate,
    pub total_price: Price,
    pub total_cubicage: Cubicage,
}

// -------------------FOR DELIVERY---------------------------------

#[derive(Debug,Clone,Default,Serialize,Deserialize)]
pub struct Delivery {
    pub danfe: Vec<DANFE>,
    pub key: Vec<Key>,
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
        self.concat_bonus();

    }

    fn calculate_price_for_each_delivery(&mut self){
        if self.total_cubicage <= 0.0 { 
            return;
        }

        for delivery in &mut self.deliveries {
            delivery.price = round_price(self.total_price*(delivery.cubicage/self.total_cubicage));
        }
    }

    fn calculate_total_cubicage(&mut self){
        self.total_cubicage = self.deliveries
            .iter()
            .map(|delivery| delivery.cubicage)
            .reduce(|a,b| a + b)
            .expect("Failed on get total value");
    }

    fn concat_bonus(&mut self){
        let mut names = HashMap::<String, usize>::new();
        let mut to_remove = Vec::<usize>::new();

        let mut new_data = self.deliveries.clone();

        for (index,delivery) in self.deliveries.iter().enumerate(){

            match names.get(&delivery.to){
                Some(value) => {
                    let first_delivery = &mut new_data[*value];
                    first_delivery.price += delivery.price;
                    first_delivery.quantity += delivery.quantity;
                    first_delivery.cubicage += delivery.cubicage;

                    first_delivery.key = vec![first_delivery.key.clone(), delivery.key.clone()].concat();
                    first_delivery.danfe = vec![first_delivery.danfe.clone(), delivery.danfe.clone()].concat();
                    to_remove.push(index);
                },
                None => {
                    names.insert(delivery.to.clone(), index);
                    continue;
                }
            };
        }


        self.deliveries = new_data.iter()
                .enumerate()
                .filter(|(i,_)| !to_remove.contains(i))
                .map(|(_, item)| item.clone())
                .collect::<Vec<_>>();


    }
}


