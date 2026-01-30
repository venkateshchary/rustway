/*
Using a hash map and vectors,
 create a text interface to allow a user to add employee names to a department in a company;
  for example, “Add Sally to Engineering” or “Add Amir to Sales.”
   Then, let the user retrieve a list of all people in a department or all people in the company by department,
    sorted alphabetically.
 */
use std::collections::HashMap;

pub fn company_interface(user_input : &str){

    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    // user_input: Add Sally to Engineering
    let v: Vec<&str> = user_input.split("to").collect();
    println!("{:?}", v);
    let department = v.get(1).unwrap().trim();
    let name_splitter = v.get(0).unwrap().trim();
    let parts: Vec<&str> = name_splitter.split_whitespace().collect();
    println!("{:?}", parts);
    let user_name = parts.get(1).unwrap();
    println!("{:?}", user_name);
    println!("{:?}", department);
    company.entry(department.to_string()).or_insert(vec![]).push(user_name.to_string());
    println!("company: {:?}", company);

}

pub fn better_way_company(input: &str, company: &mut HashMap<String,Vec<String>>){
    println!("input string given: {:?}", input);
    let splitter: Vec<&str> = input.split_whitespace().collect();
    println!("splitter: {:?}", splitter);

    match splitter.as_slice(){
        ["Add", name, "to", department] =>{
            company.entry(department.to_string()).or_insert(vec![]).push(name.to_string());
        }
        ["List", department]=>{
            if let Some(employees) = company.get(*department){
                println!("department: {:?}", department);
                for i in employees.iter(){
                    println!("{}", i);
                }
            } else {
                println!("department not found");
            }
        }
        ["List", "ALL"]=>{
            let mut departments: Vec<&String> = company.keys().collect();
            departments.sort();
            for dept in departments{
                println!("each department {:?}", dept);
                let mut  employees = company.get(dept).unwrap().clone();
                employees.sort();
                for e in employees{
                    println!("{}", e);
                }

            }

        }
        other =>{
            println!("invalid command: {:?}", other);
        }

    }
}