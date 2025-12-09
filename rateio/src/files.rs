use std::fs;
use std::path::PathBuf;

pub fn get_xml_files(dir:&PathBuf) -> Vec<PathBuf> {
    let read_data = fs::read_dir(dir).expect("Failed on read dir"); 
    let mut files : Vec<PathBuf> = read_data
        .map(|file| file.expect("Failed on get file").path())
        .filter(|file| file.to_str().unwrap().find(".xml").is_some()).collect();
    files.sort();
    files
}
