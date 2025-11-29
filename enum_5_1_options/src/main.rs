#[derive(Debug)]
enum JobStatus{
    QUEUED,
    CANCELLED {error_message:String, error_code:i32},
    FAILED(String),
    SUCCESS(String),
}

#[derive(Debug)]
struct Jobs{
    id: i32,
    name: String,
    status: Option<JobStatus>, // this will do the magic
}

/*
 Options by default has 2 variants
    1. None
    2. Some(T) T is generic type assume like (i32,string,float ...)
 */


fn main() {
    println!("Hello, world!");
    let job_1 = Jobs{
        id:1234,
        name: String::from("data_loader_us"),
        status: Some(JobStatus::SUCCESS(String::from("test"))),
    };
    println!("job_1: {:?}", job_1);
    let job_2 = Jobs{
        id:1235,
        name: String::from("data_loader_india"),
        status: None,
    };
    println!("job_2: {:?}", job_2);
    let job_3 = Jobs{
        id:1236,
        name: String::from("data_loader_uk"),
        status: Some(JobStatus::QUEUED),
    };
    println!("job_3: {:?}", job_3);
    let job_4 = Jobs{
        id:1237,
        name: String::from("data_loader_canada"),
        status: Some(JobStatus::FAILED("data not found".to_string())),
    };
    println!("job_4: {:?}", job_4);
    let job_5 = Jobs{
        id:1238,
        name: String::from("data_loader_russia"),
        status: Some(JobStatus::CANCELLED{error_message:String::from("resources are not present"), error_code:404}),
    };
    println!("job_5: {:?}", job_5);
    /*
     There are some implementation are present for the Option
        1. for verifying the created one is options is None or Some
     */

    // support you want to check the job5 status is Some job status or None

    if job_5.status.is_some(){
        println!("job_5 status is Some");
    }
    else if  job_5.status.is_none(){
        println!("job_5 status is None");
    }

    /*
        ******************************
        WE CAN USE Option in Variables
        ******************************
     */
    let name: Option<String> = Some("air".to_string());
    let x: Option<String> = Some("ownership".to_string());
    //   Normal way of representing the string
    let first_name: String = String::from("Jockery");
    println!("name: {:?}, x: {:?}, first_name: {:?}", name, x, first_name); // name: Some("air"), x: Some("ownership"), first_name: "Jockery"

    let amount: Option<i32> =  Some(4554);
    let salary: Option<i32> =  None; // here we can use Some or None because we defined variable type as Option
    println!("amount: {:?}, salary: {:?}", amount, salary); // amount: Some(4554), salary: None

    println!("name is type: {:?}", x.is_some()); // name is type: true

    /*
    this doesn't work
    if first_name.is_some(){ // no method found in String
        println!("first_name already set");
    }else {
        println!("first_name set");
    }
     Analogy
    In python we have something like
    x= 10
    if x is not None:
        print("x has some value")
     else:
        print("x is none")
    */

    let jan_month_salary : i32 = 30000;
    let jan_month_expenditure: Option<i32> =  Some(1000);
    let jan_month_expenditure_no: Option<i32> =  None;
    /* Use case:2
    There is a case like if there is no expenditure on this jan month and may be there is expenditure
        if you want to know the balance then in that case we need to remove the expenditure from the salary
    let balance: i32 = jan_month_salary- jan_month_expenditure;
    It will give you error  no implementaion for i32+Option<i32>
    Option<i32> (which can be Some(i32) or None)
    ****************** we could use the unwrap_or method here
    */

    let balance: i32 = jan_month_salary - jan_month_expenditure.unwrap_or(10000);
    let balance_no = jan_month_salary-jan_month_expenditure_no.unwrap_or_default();
    println!("balance: {:?}, balance_no: {:?}", balance, balance_no);





}
