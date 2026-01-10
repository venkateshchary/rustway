mod commonly_used_methods;

use commonly_used_methods::{truncate, first, remove_by_index, pop_from_vector, reverse};

use rand::Rng;
#[derive(Debug)]
struct User{
    username: String,
    email: String,
    age: i32,
    is_active: bool,
}

impl User{
    fn new(username: String, email: String, age: i32, is_active_param: bool) -> Self{
        Self{username, email, age, is_active:is_active_param }
    }
}

fn main() {
    println!("Hello, world!");
    let v = vec![1,2,3,4,5];

    // another way
    let mut v1:Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);
    v1.push(4);
    v1.push(1);
    println!("{:?} || {:?}",v1, v);

    // How to access the elements of vector

    let first_element = v1[0]; //if it doesn't exist exception/panic in main thread
    let first_element_with_reference = &v1[0];
    let second_element = v1.get(1); // return value or None if it doesn't exist
    println!(" first element: {:?}", first_element);
    println!(" second element: {:?}", second_element);
    println!(" first element with reference: {:?}", first_element_with_reference);

    let second_element_with_unwrap = v1.get(1).unwrap(); // unwrap gives reference &i32
    let second_element_with_deref_pointer = *v1.get(1).unwrap(); // dereference * will give actual value i32
    println!(" second element after deref: {:?}",second_element_with_deref_pointer);
    println!(" second element dereference: {:?}", second_element_with_unwrap);



    /*
        unwrap on get with index doesn't exist

     */

    // let unwrap_index_wont_exist = v1.get(103).unwrap();
    // println!(" unwrap_index_wont_exist : {:?}",unwrap_index_wont_exist);
    /*
    Error below we got
    thread 'main' panicked at src\main.rs:32:47:
        called `Option::unwrap()` on a `None` value
        stack backtrace:
           0: std::panicking::begin_panic_handler
     */

    let index_wont_exist = match v1.get(103){
        Some(value) => Some(value),
        None => None
    };
    println!("element after index_wont_exist: {:?}",index_wont_exist);


    // matching the elements
    if v1.get(0) == v1.get(4) {
        println!("both are equal");
    }else{
        println!("not equal");
    }

    if v1[0] == v1[2] {
        println!("both are equal");
    }else{
        println!("not equal");
    }
    let hundred_value = match v1.get(1){
        Some(value) => *value,
        None => 11111
    };
    println!("hundred_value is {:?}", hundred_value);
    v1.push(100);
    println!("v1 is {:?}", v1);

    /*
        for loop
     */
    for each_element in v1.iter(){
        println!("each element: {:?}", each_element);
    }


    /*
        string type vector
     */
    println!("-------------------------------------- string name vector-------");
    let mut names_vec: Vec<String> = Vec::new();
    names_vec.push(String::from("Shiva"));
    names_vec.push(String::from("Rama"));
    names_vec.push(String::from("Sathya"));
    println!("string name vector{:?}",names_vec);

    // let's access each element from vector string fields
    let first_name = &names_vec[0];
    println!("first_name: {:?}",first_name);
    println!("first_name: {:?}",*first_name);
    println!("first_name: {:?}",names_vec[0]);


    // we are borrowing the value because string is stored in heap memory, we can't copy like primitive data type
    // like we did in integer vector type let v = vec![1,2,4]; let v_first_element = v[0]

    /*
        Using random
     */
    let mut rng = rand::rng();
    let n = rng.random_range(1..=10);
    println!("Random number between 1 and 10: {}", n);

    /*
        Using for loop to create a vector with 10 numbers
     */
    println!("---------for loop with vector --------------");

    let mut ten_numbers: Vec<i32> = Vec::new();
    for number in 0..10{
        ten_numbers.push(number);
    }
    println!("ten_numbers: {:?}",ten_numbers);

    println!("-------------- collect ----------");
    /*
        Give me a vector containing numbers from 0 to 9
     */

    let mut v_range:Vec<i32> = (0..=50).collect();
    println!("v_range: {:?}",v_range);

    for i in 0..=10{
        println!("i = {:?} and v_range_value: {:?}",i, v_range[i]);
    }
    let mut sum = 0;
    for i in &mut v_range{
        println!("i = {:?}", i);
        println!("{:?}", *i);
        sum +=*i;

    };
    println!("sum is {:?}",sum);

    /*
        iterator with enumerate

     */
    // let's say keep only odd number in vector
    for (index, value) in v_range.iter().enumerate(){
        println!("index = {:?}, value: {:?}",index,value);
        if value %2 ==0{
            println!("value is divisible by 2");
            // v_range.remove(index); this won't work see readme.md
        }
    }

    /*
        let's use the retain
        our object is to remove the even numbers from a vector
            - don't create a new one
     */
    v_range.retain(|value| value%2!=0);
    println!("v_range: {:?}",v_range);

    /*
        let's use the bigger example
     */
    let age = [30,30,20,25];
    let user_names = ["Alice","Bob","Charlie","Dave"];
    let email = [String::from("alice@gmail.com"),
        String::from("bob@gmail.com"),String::from("charlie@gmail.com"),
        String::from("dave@gma_il.com")];
    println!("{:?}",email[0]);
    let is_active=[true, false, true, true];
    println!("{:?}",is_active);
    println!("{:?}", email);
    println!("{:?}", age);
    println!("{:?}", user_names);

    let mut user_vector:Vec<User> = Vec::new();
    for i in 0..user_names.len(){
        println!("user name: {:?}",user_names[i]);
        user_vector.push(User::new(
            user_names[i].to_string(),
            email[i].clone(), // clone used because of string borrowing in iteration we can't pull
            age[i],is_active[i]))

    }
    println!("user_vector: {:?}",user_vector);

    /*
        let's apply the retain
        get only is_active and age is >=30
     */

    user_vector.retain(|user|
    user.is_active && user.age>=30);
    println!("user_vector after retain: {:?}",user_vector);
    println!("length: {:?}", user_vector.len());
    println!("capacity: {:?}", user_vector.capacity());

    println!("v_range: {:?}",v_range); //[1,3,5,7,9]
    /*
    truncate to specific length
     */
    // v_range.truncate(8); // [1,3,5,7]
    println!("v_range: {:?}",v_range);

    truncate(&mut v_range, 9);
    println!("v_range: {:?}",v_range);
    let  v_first: Vec<i32> = Vec::new();
    /*
        2 ways we can write it
        if we send empty vector
            option1: let first_number: Option<&i32> = first(&v_range);
              return:  it will return None
            option2: Won't execute
                Because if let matches the condition
                if let Some(first)
     */
    let first_number = first(&v_first);
    println!("first_number: {:?}",first_number);

    if let Some(first) = first(&v_first){
        println!("first: {:?}", first);
    }
    /*
        remove particular element from the vector using the remove by index
     */
    println!("------------------ Remove --------");
    let index_to_be_removed: usize = 1;
    if let Some(removed_value) = remove_by_index(&mut v_range, index_to_be_removed){
        println!("removed_value: {:?} at index: {:?}", removed_value, index_to_be_removed);
    }else {
        println!("removed_value not found");
    }
    /*
        pop
     */
    println!("-----------------pop -----------------");
    println!("v_range: {:?}",v_range);
    // It will remove the last value from a vector
    if let Some(removed_value) = pop_from_vector(&mut v_range){
        println!("removed_value: {:?}", removed_value);
    }

    println!("---------------------REVERSE VECTOR ------------------");
    reverse(&mut v_range);
    println!("v_range: {:?}",v_range);
    for i in 0..v_range.len(){
        println!("v_range[{}]: {:?}",i,v_range[i]);
    }


}

