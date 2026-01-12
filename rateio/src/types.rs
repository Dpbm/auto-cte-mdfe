use std::collections::HashMap;
use std::rc::{Rc,Weak};
use std::cell::RefCell;
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
   EncodingXMLError(EncodingError),
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
    pub errors: Vec<String>,
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

pub type Loads = HashMap<Carrier, LoadsDataByCarrier>;

#[derive(Debug,Clone,Default,Serialize,Deserialize)]
pub struct LoadsDataByCarrier{
    pub loads: LoadsByNumberData,
    pub sequence: Vec<LoadNumber>
}

pub type LoadsByNumberData = HashMap<LoadNumber, Load>;

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


// -------------------DANFES SEQUENCE HOLDER--------------------------

type LinkedListElement = RefCell<Option<Rc<Node>>>;
type LinkedListElementBackwards = RefCell<Option<Weak<Node>>>;
type DANFENumber = u128;

#[derive(Debug)]
pub struct Node{
    pub value: DANFE,
    pub load: LoadNumber, 
    value_number: DANFENumber,
    pub next: LinkedListElement,
    pub previous: LinkedListElementBackwards
}

#[derive(Debug)]
pub struct LinkedList {
    pub head: LinkedListElement,
}

// -------------------IMPLEMENTATIONS---------------------------------

impl LinkedList {

    fn add_head(&mut self, node:Node){
        self.head = RefCell::new(Some(Rc::new(node)));
    }

    fn switch_head(&mut self, mut node:Node){

        let old_head = self.head.borrow().clone();
        if let Some(head) = old_head{ 
            node.next = Some(Rc::clone(&head)).into();
            let node_pointer = Rc::new(node);
            *head.previous.borrow_mut() = Some(Rc::downgrade(&node_pointer)).into();
            *self.head.borrow_mut() = Some(node_pointer).into(); 
                                                                
        }
        else { return; } //TODO: return an error maybe
    }

    fn switch_tail(&mut self, last_node:&Rc<Node>,  mut node:Node){

        node.previous = Some(Rc::downgrade(last_node)).into();
        *last_node.next.borrow_mut() = Some(Rc::new(node)).into();
    }

    fn insert_middle(&mut self, middle_node:&Rc<Node>, mut node:Node){
        node.previous = Some(Rc::downgrade(middle_node)).into();

        let old_pointer_middle_node_next = middle_node.next.borrow().clone();
        if let Some(middle_node_next) = &old_pointer_middle_node_next{
            node.next = Some(Rc::clone(middle_node_next)).into();

            let new_node_pointer = Rc::new(node);
            *middle_node_next.previous.borrow_mut() = Some(Rc::downgrade(&new_node_pointer)).into();
            *middle_node.next.borrow_mut() = Some(new_node_pointer).into();
        }else{
            let new_node_pointer = Rc::new(node);
            *middle_node.next.borrow_mut() = Some(new_node_pointer).into();
        }

    }


    pub fn add_between(&mut self, value:DANFE, load:LoadNumber) -> Result<(), ParseErrors> {

        let value_danfe_parsed = value.clone().as_str().parse::<u128>()?; 

        let new_node = Node{
            value: value.clone(),
            load: load,
            value_number: value_danfe_parsed,
            next: None.into(),
            previous: None.into()
        };
        
        let old_head = self.head.borrow().clone();
        if let Some(head) = old_head {
            if head.value_number > new_node.value_number{
                self.switch_head(new_node);
                return Ok(()); }

            let mut current_node = head;
            loop{
                let current_node_next_ref = current_node.next.borrow().clone();
                if let Some(next_node) = current_node_next_ref {
                    
                    if next_node.value_number > new_node.value_number {
                        break;
                    }

                    current_node = next_node;
                }else{
                    self.switch_tail(&mut current_node, new_node);
                    return Ok(());
                }

            }

            self.insert_middle(&mut current_node, new_node);

            

        }else{
            self.add_head(new_node);
        }

        Ok(())

        
    }

}

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

        let mut irregular_total = 0.0;

        for delivery in &mut self.deliveries {
            let value = round_price(self.total_price*(delivery.cubicage/self.total_cubicage));
            irregular_total += value;
            delivery.price = value;
        }

        self.deliveries[0].price = (self.total_price-irregular_total) + self.deliveries[0].price;
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


impl LoadsDataByCarrier{ 
    pub fn get_correct_sequence_of_loads(&mut self){
        let mut linked_list = LinkedList{head:None.into()};
        
        for (load,val) in self.loads.iter(){
            for delivery in &val.deliveries{
                let _ = linked_list.add_between(delivery.danfe[0].clone(), *load);
            }
        }
        
        let mut loads = vec![]; 
        let mut current_node = linked_list.head.borrow().clone();
        loop{
            if let Some(value) = current_node{
                loads.push(value.load);
                current_node = value.next.borrow().clone();
            }else{
                break;
            }
        }

        self.sequence = loads;        

    }
}

#[cfg(test)]
mod tests{
    
    use super::*;
    
    #[test]
    fn test_linkedlist_add_head(){
        let mut list = LinkedList{head:None.into()};

        if let Some(_) = *list.head.borrow() {
            panic!("Should be none");
        }

        let _ = list.add_between(String::from("12345"),0);
        if let Some(data) = &*list.head.borrow(){
            assert_eq!(data.value, String::from("12345"));
        }
    }

    #[test]
    fn test_linkedlist_switch_head(){
        let mut list = LinkedList{head:None.into()};
        let _ = list.add_between(String::from("12345"),0);
        let _ = list.add_between(String::from("00001"),0);

        if let Some(data) = &*list.head.borrow(){
            assert_eq!(data.value, String::from("00001"));

            if let Some(next) = &*data.next.borrow(){
                assert_eq!(next.value, String::from("12345"));                

                let None = &*next.next.borrow() else { panic!("Should have no more values") };
                
            }

        }
    }

    #[test]
    fn test_linkedlist_add_tail(){
        let mut list = LinkedList{head:None.into()};
        let _ = list.add_between(String::from("00001"),0);
        let _ = list.add_between(String::from("12345"),0);

        if let Some(data) = &*list.head.borrow(){
            assert_eq!(data.value, String::from("00001"));

            if let Some(next) = &*data.next.borrow(){
                assert_eq!(next.value, String::from("12345"));                

                let None = &*next.next.borrow() else { panic!("Should have no more values") };
                
            }

        }

    }


    #[test]
    fn test_linkedlist_add_in_the_middle(){
        let mut list = LinkedList{head:None.into()};
        let _ = list.add_between(String::from("00001"),0);
        let _ = list.add_between(String::from("22345"),0);
        let _ = list.add_between(String::from("12345"),0);

        if let Some(data) = &*list.head.borrow(){
            assert_eq!(data.value, String::from("00001"));

            if let Some(next) = &*data.next.borrow(){
                assert_eq!(next.value, String::from("12345"));                

                if let Some(last) = &*next.next.borrow(){
                    assert_eq!(last.value, String::from("22345"));                
                }else{
                    panic!("Should have the last value!");
                }

                
            }
        }

    }


    #[test]
    fn test_hashmap_get_loads_sequence(){
        let mut data = 
            LoadsDataByCarrier{

                loads:HashMap::from([
                    (4,Load{
                        deliveries: vec![
                            Delivery{
                                danfe: vec![String::from("8")],
                                key: vec![],
                                to: String::from("D"),
                                quantity:1,
                                price: 10.0,
                                cubicage:0.3
                            }
                        ],
                        license_plate:String::from("1"),
                        total_price:10.0,
                        total_cubicage:0.0,
                    }),
                    (3,Load{
                        deliveries: vec![
                            Delivery{
                                danfe: vec![String::from("3")],
                                key: vec![],
                                to: String::from("D"),
                                quantity:1,
                                price: 10.0,
                                cubicage:0.3
                            }
                        ],
                        license_plate:String::from("1"),
                        total_price:10.0,
                        total_cubicage:0.0,
                    }),
                    (1,Load{
                        deliveries: vec![
                            Delivery{
                                danfe: vec![String::from("5")],
                                key: vec![],
                                to: String::from("D"),
                                quantity:1,
                                price: 10.0,
                                cubicage:0.3
                            }
                        ],
                        license_plate:String::from("1"),
                        total_price:10.0,
                        total_cubicage:0.0,
                    }),
                ]),
            sequence: vec![],
        };

        data.get_correct_sequence_of_loads();
        assert_eq!(data.sequence[0], 3);
        assert_eq!(data.sequence[1], 1);
        assert_eq!(data.sequence[2], 4);
    }
}
