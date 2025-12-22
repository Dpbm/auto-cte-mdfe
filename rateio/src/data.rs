use std::path::PathBuf;
use std::collections::HashMap;

use quick_xml::reader::Reader;
use quick_xml::events::{Event,BytesText};

use crate::types::*;
use crate::constants::*;
use crate::pattern;

fn update_flag(flags:&mut u8, flag:u8){
    *flags ^= flag;
}

fn check_flag(flags:&u8, flag:u8) -> bool{
    *flags^flag == 0
}

fn match_tag(tag_name:TagName, flags:&mut u8, backtrack:&mut u8) {
    match tag_name{
        DANFE_TAG => update_flag(flags, DANFE_FLAG),
        LOAD_CUBICAGE_TAG => update_flag(flags, LOAD_CUBICAGE_FLAG),
        RAZAO_SOCIAL_FIRST_TAG => update_flag(backtrack, RAZAO_SOCIAL_BACKTRACK_FLAG),
        SHIPPING_COMPANY_FIRST_TAG => update_flag(backtrack, SHIPPING_COMPANY_BACKTRACK_FLAG),
        X_NOME if check_flag(backtrack, RAZAO_SOCIAL_BACKTRACK_FLAG) => update_flag(flags, RAZAO_SOCIAL_FLAG),
        X_NOME if check_flag(backtrack, SHIPPING_COMPANY_BACKTRACK_FLAG) => update_flag(flags, SHIPPING_COMPANY_FLAG),
        QUANTITY_TAG => update_flag(flags, QUANTITY_FLAG),
        ACCESS_KEY_TAG => update_flag(flags, ACCESS_KEY_FLAG),
        _ => (),
    }
}

fn match_text(flags:&u8, text:&BytesText, tmp_data:&mut HashMap<String,String>){
    let text_data = text.decode().unwrap().to_string();
    {
        match flags{
            flags if check_flag(&flags, DANFE_FLAG) =>  
                tmp_data.insert(String::from("danfe"), text_data),
            flags if check_flag(&flags, RAZAO_SOCIAL_FLAG) =>  
                tmp_data.insert(String::from("to"), text_data),
            flags if check_flag(&flags, SHIPPING_COMPANY_FLAG) =>  
                tmp_data.insert(String::from("by"), text_data),
            flags if check_flag(&flags, LOAD_CUBICAGE_FLAG) =>  
                tmp_data.insert(String::from("info"), text_data),
            flags if check_flag(&flags, QUANTITY_FLAG) =>  
                tmp_data.insert(String::from("quantity"), text_data),
            flags if check_flag(&flags, ACCESS_KEY_FLAG) =>  
                tmp_data.insert(String::from("access_key"), text_data),
            _ => None,
        };
    }
}

fn generate_flags() -> (u8, u8){
    let flags : u8 = 0b00000000;
    /* ============CHECK FLAGS======================
     * the flags start from right to left
     * first  - DANFE
     * second - Razao social
     * third  - Shipping company
     * forth  - Load and Cubicage
     * fifth  - Quantity
     * sixth  - Access Key
     */
        
    let backtrack : u8 = 0b00000000;
    /* ===========BACKTRACK FLAGS==================
     * this one is used when you need to check tags path
     * first  - Razao Social path
     * second - Shipping Company
     */

    (flags, backtrack)
}

pub fn parse_email(email_text:&String) -> EmailData{
    let pattern_email = pattern::text::email_text();
    let mut data = HashMap::new();

    for (_, [load_number, license_plate, price]) in pattern_email.captures_iter(email_text.to_lowercase().as_str()).map(|cap| cap.extract()){
        let load_number_parsed = load_number.parse::<LoadNumber>().expect("Failed on convert load number to number");
        data.insert(
            load_number_parsed,
            EmailLoadData{
                price: price.replace(".","").replace(",",".").parse::<Price>().expect("Failed on convert price to float"),
                license_plate: license_plate.to_string()
            }
        );
    }

    data
}

pub fn parse_file(file:&PathBuf) -> (Data, Vec<Error>) {
    let mut reader = Reader::from_file(file).expect("Failed on open reader for file");
    reader.config_mut().trim_text(true);

    let (mut flags, mut backtrack) = generate_flags();

    let mut tmp_data = HashMap::new();
    let mut buffer = Vec::new();
    let mut errors = Vec::new();
    
    loop{
        match reader.read_event_into(&mut buffer){
            Err(error) => {
                errors.push(String::from(format!("Failed on read data from xml: {:?} at position {}", error, reader.error_position())));
                break;
            },
            Ok(Event::Start(tag)) => match_tag(tag.name().as_ref(), &mut flags, &mut backtrack),
            Ok(Event::End(tag)) => match_tag(tag.name().as_ref(), &mut flags, &mut backtrack),
            Ok(Event::Text(text)) => match_text(&flags, &text, &mut tmp_data),
            Ok(Event::Eof) => break,
            _ => ()
        }
        buffer.clear();
    }
    
    let mut load_number : LoadNumber = 0;
    let mut cubicage: Cubicage = 0.0;
    let mut quantity: Quantity = 0;

    match tmp_data.get("info"){
        Some(value) => {

            let pattern_load = pattern::xml::load();
            let pattern_cubicage = pattern::xml::cubicage();

            let info = value.to_lowercase();
            match pattern_load.captures(&info){
                Some(matched_value) => {
                
                    match matched_value.get(1) {
                        Some(value) => {

                            match value.as_str().parse::<LoadNumber>(){
                                Ok(parsed_value) => {
                                    load_number = parsed_value;
                                }
                                Err(error) => {
                                    errors.push(String::from(format!("Failed on parse load number: {:?}",error)));
                                }
                            }


                        }
                        None => {
                            errors.push(String::from("Couldnt get the match for load"));
                        }
                    }

                }
                None => {
                    errors.push(String::from("No matches for load"));
                }
            }

            match pattern_cubicage.captures(&info){
                Some(matched_value) => {
                
                    match matched_value.get(1) {
                        Some(value) => {

                            match value.as_str().replace(",",".").parse::<Cubicage>(){
                                Ok(parsed_value) => {
                                    cubicage = parsed_value;
                                }
                                Err(error) => {
                                    errors.push(String::from(format!("Failed on parse cubicage: {:?}",error)));
                                }
                            }


                        }
                        None => {
                            errors.push(String::from("Couldnt get the match for cubicage"));
                        }
                    }

                }
                None => {
                    errors.push(String::from("No matches for cubicage"));
                }
            }
        },
        None => {
            errors.push(String::from("No Data for Info"));
        }
    }


    match tmp_data.get("quantity") {
        Some(value) =>  {
            
            match value.parse::<Quantity>() {
                Ok(parsed_value) => {
                    quantity = parsed_value;
                },
                Err(error) => {
                    errors.push(String::from(format!("Failed on parse quantity : {:?} ", error)));
                },
            }


        },
        None => {
            errors.push(String::from("No Quantity from parsed data!"));
        }

    }


    let danfe = match tmp_data.get("danfe") {
        Some(value) => {
            value.clone()
        },
        None => {
            errors.push(String::from("No DANFE"));
            String::from("")
        }
    };
    
    let to = match tmp_data.get("to") {
        Some(value) => {
            value.clone()
        },
        None => {
            errors.push(String::from("No client data"));
            String::from("")
        }
    };
    
    let by = match tmp_data.get("by") {
        Some(value) => {
            value.clone()
        },
        None => {
            errors.push(String::from("No carrier data"));
            String::from("")
        }
    };


    (
        Data {
            danfe: danfe,
            to: to,
            by: by,
            quantity: quantity,
            load_number: load_number,
            cubicage: cubicage,
        },
        errors
    )
}

pub fn parse_multiple(files:&Vec<PathBuf>) -> (HashMap<LoadNumber, Vec<Data>>, Vec<Error>){
    
    let mut all_data = HashMap::<LoadNumber,Vec<Data>>::new();
    let mut errors = Vec::new();

    for file in files.iter(){
        let (data,parse_errors) = parse_file(&file);
        
        errors.extend(parse_errors);

        
        if !all_data.contains_key(&data.load_number){
            all_data.insert(data.load_number, vec![data]);
            continue;
        }


        if let Some(data_list) = all_data.get_mut(&data.load_number) { data_list.push(data) }
    }

    (all_data,errors)
}

//pub fn merge_data(packet:&mut Packet){
//    for (key,val) in &packet.email_data{
//        match packet.loads.get_mut(key){
//            Some(load) => {
//                let total_price = val.price;
//
//                load.license_plate = val.license_plate.clone();
//                load.total_price = total_price;
//
//                let total_cubicage = load.calculate_total_cubicage();
//
//                load.data.iter_mut().for_each(|item| item.calculate_shipping_price(total_price, total_cubicage) );
//            }
//            None => {
//                packet.errors.push(String::from(format!("Failed on get load number: {}",key)));
//            }
//        }
//    }
//}
