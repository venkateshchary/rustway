use serde::{Deserialize};
use std::{fs, error::Error};

#[derive(Debug, Deserialize)]
pub struct User{
    name: String,
    age: i32
}

// pub fn json_reader(file_path: &str) -> Result<User, std::io::Error> {
//     let file_data_str = fs::read_to_string(file_path)?;
//     println!("str data: {:?}", file_data_str);
//     let user: User =  serde_json::from_str(&file_data_str)?;
//     println!("user: {:?}", user);
//     Ok(user)
// }

pub fn json_reader(file_path: &str) -> Result<User, Box<dyn Error>> {
    let file_data: String = fs::read_to_string(file_path)?; // this one gives Ok(string) or std::io::Error
    let json_data: User = serde_json::from_str(&file_data)?; // this one gives Ok(DATA) or serde_json::Error
    Ok(json_data)

    /*

        ? works only if
        ErrorType A -> ErrorTypeB 
        you can't convert automatically from std::io::Error -> serde_json::Error

        in the resultant type if you use the 
        Result<User, std::io::Error> it satsfies the fs::read_to_string
        if you use the

        Result<User, serde_json::Error> it satisfies the serde_json error possibility only
        So the compiler will fail

        this is where  generic error comes into picture Box<dyn Error>
     */
}