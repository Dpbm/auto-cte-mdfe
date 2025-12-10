use std::collections::HashMap;

use serde::{Deserialize, Serialize};


pub type TagName<'a> = &'a [u8];

pub type LoadNumber = u32;
pub type Cubicage = f32;


#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Data {
    pub danfe: String,
    pub to: String,
    pub by: String,
    pub load_number: LoadNumber,
    pub cubicage: Cubicage,
    pub quantity: u16
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Loads{
    pub data: HashMap<LoadNumber, Vec<Data>>
}

impl Loads{
    pub fn new() -> Self{
        Loads{
            data: HashMap::new()
        }
    }

    pub fn total_cubicage(&self, load_number:LoadNumber) -> Cubicage{
        match self.data.get(&load_number){
            None => 0.0,
            Some(values) => values
                                .into_iter()
                                .map(|value| value.cubicage)
                                .reduce(|a,b| a + b)
                                .expect("Failed on get total value")
        }
    }
}

