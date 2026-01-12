use std::env;
use std::path::PathBuf;

use actix_web::{web, post, head, App, HttpResponse, HttpServer, Responder, http::StatusCode};
use actix_web::http::header;
use actix_cors::Cors;

use serde::{Serialize};

use log::error;

use rateio::data::parsing::{parse_multiple, parse_email, concat_data};
use rateio::data::text::generate_email_text;
use rateio::files::get_xml_files;
use rateio::types::{Packet,LoadNumber};

type PortNumber = u16;

struct DataState{
    data_path: PathBuf
}

#[derive(Serialize)]
struct ErrorState {
    msg: String
}

#[post("/data")]
async fn get_data(data:web::Data<DataState>, body:String) -> impl Responder {
    let path = data.data_path.clone();

    let email_data = match parse_email(&body){
        Ok(email) => { email },
        Err(error) => {
            error!("Failed on parse email: {}",error);
            return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                .json(ErrorState{msg:"Could not parse email!".to_string()});
        }
    };

    let xml_files = get_xml_files(&path);


    match parse_multiple(&xml_files){

        Ok((data, errors)) => {
            let (loads, second_errors) = concat_data(&data, &email_data);
            let mut packet = Packet::default();
            packet.loads = loads;
            packet.errors = vec![errors.clone(), second_errors.clone()].concat();

            return HttpResponse::build(StatusCode::OK)
                .json(packet);
        },
        Err(error) => {
            error!("Failed on parse email: {}",error);
            return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                .json(ErrorState{msg:"Could not parse data!".to_string()});
        }
    }

}

#[head("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init();

    let host:String = match env::var("HOST"){
        Ok(value) => value,
        Err(e) => panic!("Failed on get HOST env: {}", e)
    };

    let port:PortNumber = match env::var("PORT"){
        Ok(value) => value.parse::<PortNumber>().unwrap(),
        Err(e) => panic!("Failed on get PORT env: {}", e)
    };

    let data_path: PathBuf = match env::var("DATA_PATH"){
        Ok(value) => PathBuf::from(value),
        Err(e) => panic!("Failed on get DATA_PATH env: {}", e)
    };

    let state = web::Data::new(
        DataState{
            data_path : data_path
        }
    );

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin_fn(|origin,_req_head|{
                        origin.as_bytes().starts_with(b"http://localhost")
                    })
                    .allowed_methods(vec!["HEAD", "POST"])
                    .allowed_header(header::CONTENT_TYPE)
                    .block_on_origin_mismatch(false)
                    .max_age(3600),
            )
            .app_data(state.clone())
            .service(health)
            .service(get_data)
    })
    .bind((host, port))?
    .run()
    .await
}
