use std::env;
use std::path::PathBuf;

use actix_web::{web, get, head, App, HttpResponse, HttpServer, Responder};

use rateio::data::parse_multiple;
use rateio::files::get_xml_files;
use rateio::types::Loads;

type PortNumber = u16;

struct DataState{
    data_path: PathBuf
}

#[get("/data")]
async fn get_data(data:web::Data<DataState>) -> impl Responder {
    let path = data.data_path.clone();
    let xml_files = get_xml_files(&path);
    let mut loads = Loads::new();
    parse_multiple(&xml_files, &mut loads);
    web::Json(loads)
}

#[head("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
            .app_data(state.clone())
            .service(health)
            .service(get_data)
    })
    .bind((host, port))?
    .run()
    .await
}
