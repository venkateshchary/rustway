mod text_splitter;
mod strings_to_pig_latin;
mod company;
mod struct_as_a_key;

use text_splitter::{text_splitter_to_hashmap};
use strings_to_pig_latin::{convert_to_pig_latin,
                           better_way_computing_pig_latin,
                           combined_logic_computing_pig_latin
};
use company::{company_interface,
              better_way_company
};

use struct_as_a_key::{ create_user};

use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    /*
        creating a new hashmap
     */
    let mut students:HashMap<String, i32> = HashMap::new();
    students.insert(String::from("Jack"), 24);
    students.insert(String::from("Jessey"), 35);
    students.insert(String::from("Maria"), 49);

    println!("students: {:?}", students); // {"Jessey": 35, "Jack": 24, "Maria": 49}

    /*
        Accessing Values in a Hash Map
     */
    let jack_student = String::from("Jack");
    println!("jack_student exam scores: {:?}", students.get(&jack_student)); // return type is Option<&i32> Some(42)

    // using unwrap you will get actual score type i32
    println!("jack_student exam scores: {:?}", students.get(&jack_student).copied().unwrap_or_default()); //42
    let unknown_user = String::from("unknown");

    if let Some(x) = students.get(&unknown_user) {
        println!("student exam scores: {:?}", x);
    }
    else {
        println!("no such user: {:?}", unknown_user);
    }

    // unwrap_or(default_value)
    let unknow_student_score = students.get(&unknown_user).copied().unwrap_or(9);
    println!("unknown student score: {:?}", unknow_student_score);

    /*
        Iterating over the hash map
     */

    for (key, value) in &students{
        println!("student name: {:?}, score: {:?}", key, value);
    }

    /*
        Ownership
     */
    println!("------- defining owned hash map -------------");
    let mut map:HashMap<String, String> = HashMap::new();
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    map.insert(field_name, field_value);
    println!("map: {:?}", map);
    // println!("field_value: {:?}", field_value); this will give error like 'value is borrowed here after move'
    println!("------------- Defining borrow hash map ---------------");
    let mut borrow_map:HashMap<&String, &String> = HashMap::new();
    let name: String = String::from("James");
    let address: String = String::from("Hyderabad India");
    borrow_map.insert(&name, &address);
    println!("borrow_map: {:?}", borrow_map);
    println!("address: {:?} | name: {:?}", address, name);

    /*
        use case: If the key doesn't exist then insert
     */
    println!("-------- if key exist --------------------");
    let mut vegetables:HashMap<String, i32> = HashMap::new();
    vegetables.insert(String::from("Tomato"), 24);
    vegetables.insert(String::from("LadyFinger"), 35);
    vegetables.insert(String::from("Onions"), 50);
    println!("vegetables: {:?}", vegetables);

    let tomato: String = String::from("Tomato");
    if vegetables.contains_key(&tomato) {
        println!("vegetable already exists: {:?}", vegetables.get(&tomato).unwrap());
    }
    else {
        vegetables.insert(tomato, 24); // even the else condition won't execute rust will take
        // all possibilities  here we are used to insert the string so ownership is transferred
        // we can't use the tomato string variable no where
    }

    /*
        Remove one key value pair from HashMap
     */
    println!("--------- Remove from hashmap ------------------");

    // vegetables.remove(&tomato); // this one will give error

    /*
        get only keys from hash map
     */
    println!("-------- get all keys ---------");
    for key in vegetables.keys() {
        println!("key: {:?}", key);
    }
    println!("vegetables: {:?}", vegetables.keys());

    /*
        get only values from hash map
     */
    println!("-------- get all values -----------");
    for value in vegetables.values() {
        println!("value: {:?}", value);
    }
    let carrot = String::from("Carrot");
    if vegetables.contains_key(&carrot) {
        println!("vegetable already exists: {:?} ||value {:?}", &carrot, vegetables.get(&carrot));
    } else {
        println!("inserting new vegetable");
      vegetables.insert(String::from("carrot"), 40);
    }
    /*
        in the same context if it is already exists.
        I wanted to update the value
     */
       if let Some(value) = vegetables.get_mut(&carrot) {
           println!("vegetable exists: {:?}", value);
           *value = 80;
    }
    println!("-------- ENTRY -------------");
    // Using Entry
    let spinach = String::from("spinach");
    let cabbage = String::from("cabbage");

    // option 1: if not exist insert
    vegetables.entry(spinach).or_insert(60);

    println!("vegetables: {:?}", vegetables); // {"carrot": 40, "LadyFinger": 35, "Tomato": 24, "spinach": 60, "Onions": 50}

    // option 2: if exist modify the update value
    vegetables.entry("spinach".to_string()).and_modify(|v| *v += 2);

    println!("vegetables: {:?}", vegetables); // vegetables: {"spinach": 62, "Tomato": 24, "LadyFinger": 35, "Onions": 50, "carrot": 40}

    // option 3: if not exists then insert or if exists update it
    println!(" if not exists then insert or if exists update it");
    vegetables.entry(cabbage).and_modify(|v| *v += 2).or_insert(20);

    println!("vegetables: {:?}", vegetables); // {"carrot": 40, "Tomato": 24, "spinach": 62, "LadyFinger": 35, "Onions": 50, "cabbage": 20}

    for (key, value) in &vegetables {
        println!("key: {:?}, value: {:?}", key, value);
    }
    println!("--------- text splitter with hashmap----");
    println!("splitter return : {:?}",text_splitter_to_hashmap());
    let phrase: String = "hash maps will provide a large amount of functionality necessary in programs".to_string();

    // convert_to_pig_latin(&phrase);
    better_way_computing_pig_latin(&phrase);

    // better way of writing code improved version
    println!("-- Input: {:?}", &phrase);
    let output:String = combined_logic_computing_pig_latin(&phrase);
    println!("output: {:?}", output);

    // company
    let user_input1 = "Add Sally to Engineering";
    let user_input2: &str = "Add Molly to Sales";
    company_interface(&user_input1);
    company_interface(&user_input2);

    let mut company_hmap:HashMap<String,Vec<String>> = HashMap::new();
    better_way_company(&user_input2, &mut company_hmap);
    better_way_company(&user_input2, &mut company_hmap);
    println!("company: {:?}", company_hmap);
    let list_departments = "List Sales";

    better_way_company(&list_departments, &mut company_hmap);
    println!("create user: {:?}", create_user());
}
