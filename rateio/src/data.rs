

mod flags{
    pub fn generate_flags() -> (u8, u8){
        let flags : u8 = 0b00000000;
        /* ============CHECK FLAGS======================
         * the flags start from right to left
         * first  - DANFE
         * second - Razao social
         * third  - Shipping company
         * forth  - Load and Cubicage fifth  - Quantity
         * sixth  - Access Key
         */
            
        let backtrack : u8 = 0b00000000;
        /* ===========BACKTRACK FLAGS==================
         * this one is used when you need to check tags path first  - Razao Social path
         * second - Shipping Company
         */

        (flags, backtrack)
    }

    pub fn update_flag(flags:&mut u8, flag:u8){
        *flags ^= flag;
    }

    pub fn check_flag(flags:&u8, flag:u8) -> bool{
        *flags&flag == flag
    }

}

mod tags{
    use std::collections::HashMap;
    use quick_xml::events::BytesText;
    use quick_xml::encoding::EncodingError;

    use crate::types::*;
    use crate::constants::*;
    use super::flags;

    pub fn match_tag(tag_name:TagName, flags:&mut u8, backtrack:&mut u8) {
        match tag_name{
            DANFE_TAG => flags::update_flag(flags, DANFE_FLAG),
            LOAD_CUBICAGE_TAG => flags::update_flag(flags, LOAD_CUBICAGE_FLAG),
            RAZAO_SOCIAL_FIRST_TAG => flags::update_flag(backtrack, RAZAO_SOCIAL_BACKTRACK_FLAG),
            SHIPPING_COMPANY_FIRST_TAG => flags::update_flag(backtrack, SHIPPING_COMPANY_BACKTRACK_FLAG),
            X_NOME => {
                if flags::check_flag(backtrack, RAZAO_SOCIAL_BACKTRACK_FLAG) {
                    flags::update_flag(flags, RAZAO_SOCIAL_FLAG);
                }
                if flags::check_flag(backtrack, SHIPPING_COMPANY_BACKTRACK_FLAG) {
                    flags::update_flag(flags, SHIPPING_COMPANY_FLAG);
                }
            },
            QUANTITY_TAG => flags::update_flag(flags, QUANTITY_FLAG),
            ACCESS_KEY_TAG => flags::update_flag(flags, ACCESS_KEY_FLAG),
            _ => (),
        }
    }

    pub fn match_text(flags:&u8, text:&BytesText, tmp_data:&mut HashMap<String,String>) -> Result<(), EncodingError>{
        let text_data = text.decode()?.to_string();
        if flags::check_flag(&flags, DANFE_FLAG) {
            tmp_data.insert(String::from("danfe"), text_data.clone());
        }
        if flags::check_flag(&flags, RAZAO_SOCIAL_FLAG) {
            tmp_data.insert(String::from("to"), text_data.clone());
        }
        if flags::check_flag(&flags, SHIPPING_COMPANY_FLAG) {
            tmp_data.insert(String::from("by"), text_data.clone());
        }
        if flags::check_flag(&flags, LOAD_CUBICAGE_FLAG) {
            tmp_data.insert(String::from("info"), text_data.clone());
        }
        if flags::check_flag(&flags, QUANTITY_FLAG) {
            tmp_data.insert(String::from("quantity"), text_data.clone());
        }
        if flags::check_flag(&flags, ACCESS_KEY_FLAG) {
            tmp_data.insert(String::from("access_key"), text_data.clone());
        }

        Ok(())
    }
}


pub mod text{
    use crate::types::LoadNumber;

    pub fn generate_email_text(loads:&Vec<LoadNumber>) -> String{
        if loads.len() <= 0{
            return String::from("");
        }


        let mut text = String::from("Segue em anexo CT-e e MDF-e ");
        if loads.len() > 1 {
            let last_load_index = loads.len()-1;
            let loads_concat = loads[..last_load_index]
                                    .iter()
                                    .map(|&v| v.to_string())
                                    .collect::<Vec<String>>().join(", ");
            text.push_str(format!("das cargas {} e {}.\n", loads_concat, loads[last_load_index]).as_str());
        }
        else {
            text.push_str(format!("da carga {}.\n", loads[0]).as_str());
        }
        
        text.push_str("att.");
        
        return text;
    }
}


pub mod parsing{
    use std::collections::HashMap;
    use std::path::PathBuf;

    use quick_xml::events::Event;
    use quick_xml::Reader;

    use crate::pattern;
    use crate::types::*;

    use super::*;

    pub fn parse_email(email_text:&String) -> Result<EmailData, ParseErrors>{
        let pattern_email = pattern::text::email_text();
        let mut data = HashMap::new();

        for (_, [load_number, license_plate, price]) in pattern_email.captures_iter(email_text.to_lowercase().as_str()).map(|cap| cap.extract()){
            let load_number_parsed = load_number.parse::<LoadNumber>()?;
            data.insert(
                load_number_parsed,
                EmailLoadData{
                    price: price.replace(".","").replace(",",".").parse::<Price>()?,
                    license_plate: license_plate.to_string()
                }
            );
        }


        Ok(data)
    }

    pub fn parse_file(file:&PathBuf) -> Result<(Data, Vec<Error>), ParseErrors> {
        let mut reader = Reader::from_file(file)?;
        reader.config_mut().trim_text(true);

        let (mut flags, mut backtrack) = flags::generate_flags();

        let mut tmp_data = HashMap::new();
        let mut buffer = Vec::new();
        let mut errors = Vec::new();
        
        loop{
            match reader.read_event_into(&mut buffer){
                Err(error) => {
                    errors.push(String::from(format!("Failed on read data from xml: {:?} at position {}", error, reader.error_position())));
                    break;
                },
                Ok(Event::Start(tag)) => tags::match_tag(tag.name().as_ref(), &mut flags, &mut backtrack),
                Ok(Event::End(tag)) => tags::match_tag(tag.name().as_ref(), &mut flags, &mut backtrack),
                Ok(Event::Text(text)) => tags::match_text(&flags, &text, &mut tmp_data)?,
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
        
        let key = match tmp_data.get("access_key") {
            Some(value) => {
                value.clone()
            },
            None => {
                errors.push(String::from("No NF-e Key"));
                String::from("")
            }
        };


        Ok((
            Data {
                danfe: danfe,
                to: to,
                by: by,
                quantity: quantity,
                load_number: load_number,
                cubicage: cubicage,
                key: key
            },
            errors
        ))
    }

    pub fn parse_multiple(files:&Vec<PathBuf>) -> Result<(MultipleData, Vec<Error>), ParseErrors>{
        
        let mut all_data = MultipleData::new();
        let mut errors = Vec::new();

        for file in files.iter(){
            let (data,parse_errors) = parse_file(&file)?;
            
            errors.extend(parse_errors);

            
            if !all_data.contains_key(&data.load_number){
                all_data.insert(data.load_number, vec![data]);
                continue;
            }


            if let Some(data_list) = all_data.get_mut(&data.load_number) { data_list.push(data) }
        }

        Ok((all_data,errors))
    }

    pub fn concat_data(data:&MultipleData, email_data:&EmailData) -> (Loads, Vec<Error>){
        let mut loads = Loads::new();
        let mut errors = Vec::<Error>::new();
        
        for (load_number, data_loads) in data.iter() {
            
            for d in data_loads{
                
                if !loads.contains_key(&d.by){
                    loads.insert(
                        d.by.clone(), 
                        LoadsDataByCarrier::default()
                    );
                }
                
                let load_email_data = match email_data.get(load_number){
                    Some(data) => {data},
                    None => {
                        errors.push(String::from(format!("Load {} not found on email", load_number)));
                        continue;
                    }
                };

                if let Some(carrier_loads) = loads.get_mut(&d.by) { 

                    let delivery = Delivery {
                        danfe: vec![d.danfe.clone()],
                        key: vec![d.key.clone()],
                        to: d.to.clone(),
                        quantity: d.quantity,
                        cubicage: d.cubicage,
                        ..Default::default()
                    };

                    
                    if !carrier_loads.loads.contains_key(load_number) {
                        let mut load = Load{
                            license_plate: load_email_data.license_plate.clone(),
                            total_price: load_email_data.price,
                            ..Default::default()
                        };
                        
                        load.deliveries.push(delivery);
                        carrier_loads.loads.insert(*load_number, load);
                    }else{

                        if let Some(load_data) = carrier_loads.loads.get_mut(load_number) {
                            load_data.deliveries.push(delivery);
                        };

                    }

                };

            }

        }

        for (_,data) in loads.iter_mut(){
            data.get_correct_sequence_of_loads();
            data.get_email_text();
            for (_, load) in data.loads.iter_mut(){
                load.update_load_delivery_data();
            }
        }

        (loads,errors)
    }
}




#[cfg(test)]
mod tests{
    use std::collections::HashMap;
    use std::path::PathBuf;

    use quick_xml::events::BytesText;

    use crate::constants::*;
    use crate::types::*;
    use crate::types::ParseErrors;

    use super::*;
   
    #[test]
    fn test_get_flags(){
        let (flag1, flag2) = flags::generate_flags();

        assert_eq!(flag1, 0);
        assert_eq!(flag2, 0);
    }

    #[test]
    fn test_update_flags(){
        let (mut flag1, mut flag2) = flags::generate_flags();

        flags::update_flag(&mut flag1, 1);
        assert_eq!(flag1,1);

        flags::update_flag(&mut flag2, 255);
        assert_eq!(flag2,255);
    }

    #[test]
    fn test_check_flags(){
        let (mut flag1, _) = flags::generate_flags();

        flags::update_flag(&mut flag1, DANFE_FLAG);
        assert_eq!(flags::check_flag(&flag1, DANFE_FLAG), true);
        
        flags::update_flag(&mut flag1, RAZAO_SOCIAL_FLAG);
        assert_eq!(flags::check_flag(&flag1, RAZAO_SOCIAL_FLAG), true); 
        assert_eq!(flags::check_flag(&flag1, DANFE_FLAG), true);
        
        flags::update_flag(&mut flag1, SHIPPING_COMPANY_FLAG);
        assert_eq!(flags::check_flag(&flag1, RAZAO_SOCIAL_FLAG), true); 
        assert_eq!(flags::check_flag(&flag1, DANFE_FLAG), true);
        assert_eq!(flags::check_flag(&flag1, SHIPPING_COMPANY_FLAG), true);
    }

    #[test]
    fn test_match_tag(){
        let (mut flags, mut backtrack) = flags::generate_flags();
        let mut total_flag = 0;
        let mut total_backtrack = 0;

        // no tag
        tags::match_tag(b"invalid_tag", &mut flags, &mut backtrack);
        assert_eq!(flags,total_flag);
        assert_eq!(backtrack,total_backtrack);
        
        tags::match_tag(DANFE_TAG, &mut flags, &mut backtrack);
        total_flag += DANFE_FLAG;
        assert_eq!(flags,total_flag);
        assert_eq!(backtrack,total_backtrack);
        
        tags::match_tag(LOAD_CUBICAGE_TAG, &mut flags, &mut backtrack);
        total_flag += LOAD_CUBICAGE_FLAG;
        assert_eq!(flags,total_flag);
        assert_eq!(backtrack,total_backtrack);
        
        // X_NOME without backtrack
        tags::match_tag(X_NOME, &mut flags, &mut backtrack);
        assert_eq!(flags,total_flag);
        assert_eq!(backtrack,total_backtrack);

        tags::match_tag(RAZAO_SOCIAL_FIRST_TAG, &mut flags, &mut backtrack);
        assert_eq!(flags,total_flag);
        assert_eq!(backtrack,total_backtrack+RAZAO_SOCIAL_BACKTRACK_FLAG);
        
        // X_NOME for razao social
        tags::match_tag(X_NOME, &mut flags, &mut backtrack);
        assert_eq!(flags,total_flag+RAZAO_SOCIAL_FLAG);
        assert_eq!(backtrack,total_backtrack+RAZAO_SOCIAL_BACKTRACK_FLAG);
        
        flags::update_flag(&mut flags, RAZAO_SOCIAL_FLAG);
        flags::update_flag(&mut backtrack, RAZAO_SOCIAL_BACKTRACK_FLAG);
        tags::match_tag(SHIPPING_COMPANY_FIRST_TAG, &mut flags, &mut backtrack);
        assert_eq!(flags,total_flag);
        assert_eq!(backtrack,total_backtrack+SHIPPING_COMPANY_BACKTRACK_FLAG);
        
        // X_NOME for shipping company
        tags::match_tag(X_NOME, &mut flags, &mut backtrack);
        assert_eq!(flags,total_flag+SHIPPING_COMPANY_FLAG);
        assert_eq!(backtrack,total_backtrack+SHIPPING_COMPANY_BACKTRACK_FLAG);

        // X_NOME with both flags
        flags::update_flag(&mut flags, SHIPPING_COMPANY_FLAG);
        flags::update_flag(&mut backtrack, SHIPPING_COMPANY_BACKTRACK_FLAG);
        tags::match_tag(RAZAO_SOCIAL_FIRST_TAG, &mut flags, &mut backtrack);
        tags::match_tag(SHIPPING_COMPANY_FIRST_TAG, &mut flags, &mut backtrack);
        total_backtrack += RAZAO_SOCIAL_BACKTRACK_FLAG + SHIPPING_COMPANY_BACKTRACK_FLAG;
        total_flag += RAZAO_SOCIAL_FLAG + SHIPPING_COMPANY_FLAG;
        tags::match_tag(X_NOME, &mut flags, &mut backtrack);
        assert_eq!(flags,total_flag);
        assert_eq!(backtrack,total_backtrack);

        tags::match_tag(QUANTITY_TAG, &mut flags, &mut backtrack);
        total_flag += QUANTITY_FLAG;
        assert_eq!(flags,total_flag);
        assert_eq!(backtrack,total_backtrack);

        tags::match_tag(ACCESS_KEY_TAG, &mut flags, &mut backtrack);
        total_flag += ACCESS_KEY_FLAG;
        assert_eq!(flags,total_flag);
        assert_eq!(backtrack,total_backtrack);
    }

    #[test]
    fn test_match_text() -> Result<(), ParseErrors>{
        let mut flags : u8 = 1;
        let mut data = HashMap::new();

        let base_text = "test";
        let text = BytesText::new(&base_text);

        for power in 0..=7{
            flags <<= power;
            tags::match_text(&flags, &text, &mut data)?;
        }

        for (_,v) in data.iter(){
            assert_eq!(v, base_text);
        }

        
        let all_flags : u8 = 255;
        let mut data = HashMap::new();
        tags::match_text(&all_flags, &text, &mut data)?;
        for (_,v) in data.iter(){
            assert_eq!(v, base_text);
        }

        Ok(())
    }

    #[test]
    fn test_parse_email(){
        
        let email = String::from(r#"
            carga 123456 Placa 1234asz fRetE 1.342,87
            CArgA : 345678 PlAca: 1234asz fRetE : 8.342,93
            Carga: 891234 Placa: 124-asz fRetE:1.342,87
        "#);

        let data = parsing::parse_email(&email).unwrap();
        
        let first_load = data.get(&123456).unwrap();
        assert_eq!(first_load.price, 1342.87);
        assert_eq!(first_load.license_plate, "1234asz");

        let second_load = data.get(&345678).unwrap();
        assert_eq!(second_load.price, 8342.93);
        assert_eq!(second_load.license_plate, "1234asz");

        let third_load = data.get(&891234).unwrap();
        assert_eq!(third_load.price, 1342.87);
        assert_eq!(third_load.license_plate, "124-asz");
    }
    
    #[test]
    fn test_parse_file() -> Result<(), ParseErrors>{
        let correct_file_path = PathBuf::from("./test_data/correct.xml");
        let (data,errors) = parsing::parse_file(&correct_file_path)?; 

        assert_eq!(data.danfe, "12345");
        assert_eq!(data.cubicage, 3.431);
        assert_eq!(data.to, "test");
        assert_eq!(data.by, "test3");
        assert_eq!(data.quantity, 10000);
        assert_eq!(data.load_number, 3245);
        assert_eq!(data.key, "78493");


        assert_eq!(errors.len(), 0);
        
        let wrong_file_path = PathBuf::from("./test_data/wrong.xml");
        let (data,errors) = parsing::parse_file(&wrong_file_path)?; 

        assert_eq!(data.danfe, "");
        assert_eq!(data.cubicage, 0.0);
        assert_eq!(data.to, "");
        assert_eq!(data.by, "");
        assert_eq!(data.quantity, 0);
        assert_eq!(data.load_number, 0);
        assert_eq!(data.key, "");

        assert_eq!(errors.len(), 6); // cubicage and load number are in the same tag

        Ok(())
    }

    #[test]
    fn test_concat_data() {
        let data = HashMap::from([
            (10, vec![
             Data{
                 danfe: String::from("123"),
                 to: String::from("1"),
                 by: String::from("12"),
                 quantity: 10,
                 load_number:10,
                 cubicage: 1.3,
                 key: String::from("123")
             }
            ]),
            (20, vec![
             Data{
                 danfe: String::from("1235"),
                 to: String::from("2"),
                 by: String::from("13"),
                 quantity: 100,
                 load_number:20,
                 cubicage: 1.35,
                 key: String::from("1234")
             }
            ]),
        ]); 
        
        let email = HashMap::from([
            (10, EmailLoadData{
                price: 100.0,
                license_plate: String::from("bbbd"),
            }),
            (20,EmailLoadData{
                price: 200.0,
                license_plate: String::from("ddda")
            })
        ]);
        
        let (result,errors) = parsing::concat_data(&data, &email);
        println!("{:?}", result);

        assert_eq!(errors.len(), 0);

        let from_12 = result.get("12").unwrap().loads.get(&10).unwrap();
        let from_12_seq = &result.get("12").unwrap().sequence;
        assert_eq!(from_12_seq[0], 10);
        assert_eq!(from_12_seq.len(), 1);

        assert_eq!(from_12.license_plate, "bbbd");
        assert_eq!(from_12.total_price, 100.0);
        assert_eq!(from_12.total_cubicage, 1.3);
        assert_eq!(from_12.deliveries.len(), 1);

        let from_12_deliveries = &from_12.deliveries[0];
        assert_eq!(from_12_deliveries.danfe.len(),1);
        assert_eq!(from_12_deliveries.key.len(),1);
        assert_eq!(from_12_deliveries.danfe[0],"123");
        assert_eq!(from_12_deliveries.key[0],"123");
        assert_eq!(from_12_deliveries.to,"1");
        assert_eq!(from_12_deliveries.quantity,10);
        assert_eq!(from_12_deliveries.price,100.0);
        assert_eq!(from_12_deliveries.cubicage,1.3);

        let from_13 = result.get("13").unwrap().loads.get(&20).unwrap();
        let from_13_seq = &result.get("13").unwrap().sequence;
        assert_eq!(from_13_seq[0], 20);
        assert_eq!(from_13_seq.len(), 1);

        assert_eq!(from_13.license_plate, "ddda");
        assert_eq!(from_13.total_price, 200.0);
        assert_eq!(from_13.total_cubicage, 1.35);
        assert_eq!(from_13.deliveries.len(), 1);

        let from_13_deliveries = &from_13.deliveries[0];
        assert_eq!(from_13_deliveries.danfe.len(),1);
        assert_eq!(from_13_deliveries.key.len(),1);
        assert_eq!(from_13_deliveries.danfe[0],"1235");
        assert_eq!(from_13_deliveries.key[0],"1234");
        assert_eq!(from_13_deliveries.to,"2");
        assert_eq!(from_13_deliveries.quantity,100);
        assert_eq!(from_13_deliveries.price,200.0);
        assert_eq!(from_13_deliveries.cubicage,1.35);


    }


    #[test]
    fn test_email_no_loads() {
        let text = text::generate_email_text(&vec![]);
        assert_eq!(text,String::from(""));
    }


    #[test]
    fn test_email_single_load() {
        let text = text::generate_email_text(&vec![1]);
        assert_eq!(text,String::from("Segue em anexo CT-e e MDF-e da carga 1.\natt."));
    }

    #[test]
    fn test_email_multiple_loads() {
        let text = text::generate_email_text(&vec![1,2]);
        assert_eq!(text,String::from("Segue em anexo CT-e e MDF-e das cargas 1 e 2.\natt."));
        
        let text = text::generate_email_text(&vec![1,2,3]);
        assert_eq!(text,String::from("Segue em anexo CT-e e MDF-e das cargas 1, 2 e 3.\natt."));
    }
}
