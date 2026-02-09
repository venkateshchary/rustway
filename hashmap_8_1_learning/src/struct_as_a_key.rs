// struct as a key in hashmap
use std::collections::HashMap;
/*
    creating non-primitive type as a key in hashmap
    HashMap key should have properties like Equal and PartialEqual and Hash so we derived it
    like for the entry method we usually check if it exists
    and if user1 == user 2 for these cases we need EQ/PartialEq

    debug will be used for print

    ** without these derives we can't use struct as a key in hashmap **
 */

#[derive(Debug,Eq,PartialEq,Hash)]
pub struct User{
    name: String,
    age: i32
}

impl User{

    fn new(name: String, age:i32) -> Self{
        Self{name, age
        }
    }
}

pub fn create_user() -> HashMap<User, String>{
    let mut u: HashMap<User, String> = HashMap::new();
    let user1:User = User{name:"venaktesh".to_string(), age:44};
    let user2: User = User::new("sathya".to_string(), 22);
    u.insert(user1, "hyderabad".to_string());
    u.insert(user2, "hyderabad".to_string());
    println!("user: {:?}", u);
    u
}
