mod file_reader;

#[derive(Debug)]
pub enum Filetypes{
    Text,
    Html,
    Json,
    Xml,
    Csv,
    Xls
}

pub fn verify_file_exist(file_name: &str){
    println!("File exists: {:?}", file_name);

}

pub fn verify_file_extension(file_name: &str){
    println!("File extension: {:?}", file_name);
}