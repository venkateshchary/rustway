
#[derive(Debug)]
pub enum FileTypes{
    Txt,
    Json,
    Csv

}
pub fn check_file_pattern(file_path: &str) -> Option<FileTypes>{
    println!("Checking file pattern");
    if file_path.ends_with("txt"){
        Some(FileTypes::Txt)
    }else if file_path.ends_with("json"){
        Some(FileTypes::Json)
    }else if file_path.ends_with("csv") {
        Some(FileTypes::Csv)
    }else {
        None
    }
}

pub fn is_file_exist(file_path: &str) -> bool{
    if file_path.is_empty(){
        false
    }else {
        true
    }
}