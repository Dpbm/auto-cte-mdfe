use std::path::PathBuf;
use std::collections::HashMap;

use regex::Regex;
use quick_xml::reader::Reader;
use quick_xml::events::{Event,BytesText};

use crate::types::*;
use crate::constants::*;

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



pub fn parse_file(file:&PathBuf) -> Data{
    let mut reader = Reader::from_file(file).expect("Failed on open reader for file");
    reader.config_mut().trim_text(true);

    let pattern_load = Regex::new(r"carga *:* *([0-9]+)").unwrap();
    let pattern_cubicage = Regex::new(r"cubicagem *:* *([0-9]+,[0-9]+) *m3").unwrap();
    
    let (mut flags, mut backtrack) = generate_flags();

    let mut tmp_data = HashMap::new();
    let mut buffer = Vec::new();
    
    loop{
        match reader.read_event_into(&mut buffer){
            Err(error) => {
                println!("Failed on read data from xml: {:?} at position {}", error, reader.error_position());
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

    let info = tmp_data["info"].to_lowercase();
    let load_number = pattern_load.captures(&info).unwrap().get(1).unwrap().as_str().parse::<u32>().unwrap();
    let cubicage = pattern_cubicage.captures(&info).unwrap().get(1).unwrap().as_str().replace(",",".").parse::<f32>().unwrap(); Data {
        danfe: tmp_data["danfe"].clone(),
        to: tmp_data["to"].clone(),
        by: tmp_data["by"].clone(),
        quantity: tmp_data["quantity"].parse::<u16>().unwrap(),
        load_number: load_number,
        cubicage: cubicage
    }
}

pub fn parse_multiple(files:&Vec<PathBuf>, all_data:&mut Loads){
    for file in files.iter(){
        let data = parse_file(&file);
        
        if !all_data.data.contains_key(&data.load_number){
            all_data.data.insert(data.load_number, Vec::new());
        }
        if let Some(load) = all_data.data.get_mut(&data.load_number) { load.push(data) }
    }
}


