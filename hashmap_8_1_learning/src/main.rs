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

    if vegetables.contains_key(&String::from("Tomato")) {
        println!("vegetable already exists: {:?}", vegetables.get(&String::from("Tomato")).unwrap());
    }
}
