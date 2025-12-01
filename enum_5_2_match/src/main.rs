


#[derive(Debug)]
enum APIResponse{
    Success {status_code: i32, message: String},
    Failure {status_code: i32, message: String},
    Created {message: String},
    ServerError{status_code: i32},
}

fn is_known_error_codes(error_code: i32) -> bool {
    let user_defined_error_codes = [4000,2000,700];
    let mut return_statement = false;
    println!("{:?}",user_defined_error_codes);
    for code in &user_defined_error_codes {
        if error_code == *code {
            return_statement= true
        }
    }
    return_statement
}

fn handle_response(response: APIResponse) {
    match response {
        APIResponse::Success{status_code, message} => {
        },
        APIResponse::Failure{status_code, message} => {
            /*
                Within the match flow we are calling the function whatever the function is returning
                we are returning here
             */
            println!("{:?} | status code: {:?}",message, status_code);
            println!("message: {:?}",is_known_error_codes(status_code));
            is_known_error_codes(status_code);
        }
        APIResponse::Created{message} => {
            // here we can write any logic
            println!("{}", message);
        },
        APIResponse::ServerError{status_code} => {}
    }
}

fn toddler_age_handler(age: Option<i32>) -> bool{
    match age {
        Some(age) => { // First variant
            if age<=3 && age>=1 {
                true
            }else {
                false
            }
        }
        None => false // Second Variant
    }
}

fn cricket_things(runs: i32) {
    match runs {
        4 => {
            println!("It's Four");
        },
        6 =>{
            println!("It's Sixer");
        }
        other =>{
            println!("It's Two d");
            never_gonna_happen(other);
        }
        /*
            other means "match anything and store it"
            _ mean match anything but don't BIND
         */
        // _ =>{ // this is dead code
        //     println!("It's 2 d");
        // }

    }
}

fn never_gonna_happen(runs: i32) {
    println!("Sehwag Single? Never gonna happen! {:?}", runs)
}


fn main() {
    println!("Hello, world!");
    let user_creation_api = APIResponse::Created {message: "User is created".to_string()};
    let known_error_codes = APIResponse::Failure {status_code:4000, message:"known error".to_string()};
    /*
        Now lets use the match control flow
     */
    handle_response(user_creation_api);
    handle_response(known_error_codes);

    /*
        Using the Options in the match
        As we know Options has 2 variants (Some & None)
     */
    let toddler_age: Option<i32> = Some(3);
    let toddller_age_1: Option<i32> = None;
    let toddller_age_2:Option<i32>  = Some(1);
    println!("for 3 years old kid can we call him a toddler:{:?}",toddler_age_handler(toddler_age));
    println!("for None years old kid can we call him a toddler:{:?}",toddler_age_handler(toddller_age_1));
    println!("for 1 years old kid can we call him a toddler:{:?}",toddler_age_handler(toddller_age_2));


    /*
        Let's take an example of cricket scoring things
        first over sehwag is batting
     */
    let first_ball: i32 = 4i32;
    let second_ball: i32 = 4i32;
    let third_ball: i32 = 6i32;
    let fourth_ball: i32 = 2i32;
    let fifth_ball: i32 = 6i32;
    let sixth_ball: i32 = 1i32;
    println!("{:?}",cricket_things(first_ball));
    println!("{:?}",cricket_things(second_ball));
    println!("{:?}",cricket_things(third_ball));
    println!("{:?}",cricket_things(fourth_ball));
    println!("{:?}",cricket_things(fifth_ball));
    println!("{:?}",cricket_things(sixth_ball));



}
