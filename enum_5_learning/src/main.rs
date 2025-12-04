
/*
Define ENUM
*/

#[derive(Debug)]
enum JobStatus{
    Waiting,
    Started,
    InProgress,
    Completed,
    Error,
}

// use the struct again here
#[derive(Debug)]
struct DataPipeline{
    job_id: u32,
    job_name: String,
    job_status: JobStatus
}

#[derive(Debug, Clone, Copy)]
enum IpAddress{
    V4,
    V6
}

#[derive(Debug)]
struct IpAddr{
    kind: IpAddress,
    value: (u8,u8,u8,u8),
}

impl IpAddr{

    fn get_current_ip_address(&self) -> (u8,u8,u8,u8){
        self.value
    }

    // after adding the clone and copy on top
    // of enum you will be able to get the copies instead of moves
    fn get_current_type(&self) -> IpAddress{
        self.kind
    }

}

impl DataPipeline{

    fn new(job_id: u32, job_name: &str, job_status: JobStatus) -> Self{
        Self{
            job_id,
            job_name: job_name.to_string(),
            job_status,
        }
    }
    /*
        below function doesn't work because we have not derived the clone and copy on enum
        we're directly transferring the ownership
     
        fn current_job_status(&self) -> JobStatus {
            self.job_status
        }
    */

    // rectifying it

    fn current_job_status_with_reference_only(&self) -> &JobStatus{
        &self.job_status
    }

    fn current_job_id(&self) -> u32 {
        self.job_id
    }

    /*
        Updating the job status value
     */

    fn update_job_status(&mut self, new_status: JobStatus){
        self.job_status = new_status
    }
}



fn main() {
    println!("Hello, world!");
    let waiting_job_status = JobStatus::Waiting;
    println!("waiting job status: {:?}", waiting_job_status);
    let mut job1 = DataPipeline::new(1232, "airflow1", waiting_job_status);
    println!("data pipeline job1 1: {:?}", job1);
    // println!("JOB  status: {:?}", job1.current_job_status());
    
    /*
        ERROR MESSAGE: you could clone this value
        job1.current_job_status() Doesn't work because of heap memory
        here we are returing the JobStatus type which is not a primitive type that implements `Copy`
        WE NEED TO DERIVE THE CLONE & COPY ON THE JobStatus ENUM then it will work
    */

    // For only view purpose then we can use the reference

    println!("job status current: {:?}", job1.current_job_status_with_reference_only()); // job status current: Waiting
    
    /*
        UPdating the job status
     */

    job1.update_job_status(JobStatus::Started);

    println!("job status current after update: {:?}", job1.current_job_status_with_reference_only()); // job status current: Waiting



    println!("current job_id: {:?}", job1.current_job_id());
    /*
        *************************************************************************
        ** to hold the values we are using the combination of enum with struct **
        *************************************************************************
     */

    let v4_type = IpAddress::V4;
    let ip1 = IpAddr{
        kind: v4_type,
        value: (127,0,0,1)
    };
    println!("ip1: {:?}", ip1); // ip1: IpAddr { kind: V4, value: (127, 0, 0, 1) }

    println!("current ip_address value: {:?}", ip1.get_current_ip_address()); // current ip_address value: (127, 0, 0, 1)
    /*
        This will work out due to copy trait is implemented for the tuple with all integer fields
        there is no transfer of the ownership and all the values are fixed and stored in the stack memory 
     */


    /*
        ***********************************************************
        **** Directly using the Enum to hold the values as well ***
        ***********************************************************
     */

    #[derive(Debug)]
    enum IpAddrWithValues{
        V4(String),
        V6(u8, u8, u8, u8),
        belongs_to{state: String, home: String}
    }

    
    let home = IpAddrWithValues::V4(String::from("127.0.0.1"));
    let belongs_to = IpAddrWithValues::belongs_to {state:String::from("TELANGANA"), home:String::from("India")};
    let office = IpAddrWithValues::V6(13,10,11,10);
    println!("belongs to : {:?}", belongs_to);
    println!("home: {:?} office: {:?}", home, office); // home: V4("127.0.0.1") office: V6(13, 10, 11, 10)

}
