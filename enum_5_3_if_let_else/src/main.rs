/*
    Let's combine If and Let
 */
#[derive(Debug)]
enum APIResponse{
    Failed(String),
    Success(i32),
}

fn main() {
    println!("Hello, world! Let's learn about if and let");

    let config_max: Option<i8> = Some(4i8);
    // let config_max: Option<i8> = None; // in this case it will use the wildcard
    match config_max {
        Some(max) => { // 1st arm
            println!("The maximum is configured to be: {:?}", max);

        },
        // to satisfy the match expression we need to have _
        // either you need to add None or _
        _ => {
            // making _ (underscore) arm
            println!("wild card it will match other than some value");
            ()
        }
    }

    /*
        In shorthand, we define like below using if let
        adn we are ignoring all other arms, only caring about some only
     */
    if let Some(max) = config_max {
        println!("The maximum is configured to be: {:?}", max);
    }

    /*
        Now we will use if let pattern. we only care about the Success responses
     */
    // let success_response_code: i32 = 200;
    // let member_response = APIResponse::Success(success_response_code);
    let member_response = APIResponse::Success(200i32); // these declarations both are same, just we avoided the variable declaration
    if let APIResponse::Success(code) = member_response {
        println!("The successful response is: {:?}", code);
    }
    // Let's write using the match considering the all the arms


    match member_response {
        APIResponse::Failed(msg) => println!("The error message is: {}", msg),
        APIResponse::Success(val) => println!("The successful response is: {:?}", val),
    }
    /*
        Let's consider if let else case
        suppose you declared the match case fo success api response ,but variable is having the failure state

     */

    let api_response1 = APIResponse::Failed(String::from("Something went wrong"));

    // let's write LET IF ELSE

    /*
        ******************************************************************
        if let checks one specific pattern, and else handles everything else.
        *******************************************************************

     */

    if let APIResponse::Success(val) = api_response1 {
        println!("The successful response is: {:?}", val);
    }else {
        println!("we don't support APIResponse: IT is failed one"); // we don't support APIResponse: IT is failed one
    }

}
