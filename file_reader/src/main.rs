use std::fs;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct Dog{
    name: String,
    year_born: i32,
}

impl Dog{

    pub fn born_gt_2000(&self, year: i32)-> bool{
        self.year_born > year
    }
}

fn file_reader(file_path: &str) -> Result<Vec<Dog>, Box<dyn Error>> {
    let file_data_string = fs::read_to_string(file_path)?; // io::Error
    let serialized_json: Vec<Dog> = serde_json::from_str(&file_data_string)?; // serde_json::Error
    Ok(serialized_json)
    }


fn main() {
    println!("Hello, world!");
    let dog1 = Dog{name: "chiyan".to_string(), year_born: 2000};
    let rust_to_json = serde_json::to_string(&dog1).unwrap();
    println!("after serializing from rust data to json {:?}", rust_to_json);
    // after serializing from rust data to json "{\"name\":\"chiyan\",\"year_born\":2000}"

    println!("deserializing  from json to rust data");
    let json_to_rust = serde_json::from_str::<Dog>(&rust_to_json).unwrap();

    println!("after deserializing from json to rust data :{:?}", json_to_rust);
    // after deserializing from json to rust data :Dog { name: "chiyan", year_born: 2000 }

    let file_str = file_reader("data/dogs_list.json").unwrap();
    println!("after reading from file :{:?}", file_str);
    // let str_to_rust_data = serde_json::from_str::Vec<Dog>(&file_str);
    let year = 2020;
    for dog in file_str.iter() {
        if dog.born_gt_2000(year){
            println!("dogs born on current year {:?}",dog);
        }
    }
}
