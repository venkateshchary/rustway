use std::{fmt::format, fs};
use serde::Deserialize;
use serde_json::map;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Age{
    Number(u32),
    String(String)
}

#[derive(Debug, Deserialize)]
pub struct User{
    // how to define a structure
    pub id: u32,
    pub name: String,
    pub email: String,
    pub age: Age

}

impl Age{
    pub fn as_u32(&self) -> Result<u32, String> {
        match self {
            Age::Number(n)=>{
                Ok(*n)
            }
            Age::String(s)=>{
                s.parse::<u32>().map_err(|_| format!("invalid format : {}", s))
            }
        }
    }
}

impl User{
    
    pub fn is_name_not_empty(&self,)-> bool{
        if !self.name.is_empty(){
            true
        }else{
            false
        }
    }

}

pub fn list_users_reader() -> Result<Vec<User>, Box<dyn std::error::Error>>{

    let file_string_data: String = fs::read_to_string("data/list_of_users.json")?;
    let data: Vec<User> = serde_json::from_str(&file_string_data)?;
    Ok(data)
}

pub fn reader_list_file(){
    let user_data:Result<Vec<User>, Box<dyn std::error::Error>>  = list_users_reader();
    let mut correct_user_objects:Vec<User> = Vec::new();
    match user_data{
        Ok(user_data) =>{
            for i in user_data{
                println!("i:{:?}", i);
                if i.is_name_not_empty(){
                    let age = i.age.as_u32().unwrap_or_default();
                    println!("age: {:?}", age);
                    correct_user_objects.push(i);
                }
            };
        }
        Err(error)=>{
            println!("error: {:?}", error);
        }
    }
    println!("vector final is : {:?}", correct_user_objects);

}