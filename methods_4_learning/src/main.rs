#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn create_rectangle(width: u32, height: u32) -> Self{
        Self{
            width,
            height
        }
    }

}

fn main() {
    println!("Hello, world!");
    let rect1 = Rectangle {
        width: 50,
        height: 50,
    };
    println!("rectangle is : {:?}", rect1); // rectangle is : Rectangle { width: 50, height: 50 }
    let area_of_rectangle = rect1.area();
    println!("area of rectangle: {:?}", area_of_rectangle); // area of rectangle: 2500
                                                            // full
    println!("second way : {:?}", Rectangle::area(&rect1));
    let person1 = Person {
        age: 60,
        gender: 'M',
        address: "hyderabad india".to_string(),
    };
    let person2 = Person {
        age: 40,
        gender: 'F',
        address: "pune India".to_string(),
    };
    println!("is senior citizen :{:?}", person1.is_senior_citizen());
    println!(
        "is both persons are same: {:?}",
        person1.compare_other_person_properties(&person2)
    );
    
    // Associate function call

    let person3 = Person::new(24, 'F', "Russia");
    println!("person creation using associate function: {:?}", person3); // person creation using associate function: Person { age: 24, gender: 'F', address: "Russia" }

    // Associate function to create a new rectangle

    let rect2 = Rectangle::create_rectangle(30, 40);
    println!("rectangle the new one with data we have created: {:?}", rect2);

}

#[derive(Debug)]
struct Person {
    age: u32,
    gender: char,
    address: String,
}

impl Person {
    fn is_senior_citizen(&self) -> bool {
        if self.age > 60 {
            true
        } else {
            false
        }
    }
    
    // Associate function creation

    fn new(age: u32, gender: char, address: &str) -> Person{
        Person{
            age,
            gender,
            address: address.to_string(),
        }
    }


    /*
        suppose you want to compare 2 persons
    */
    fn compare_other_person_properties(&self, other: &Person) -> bool {
        if self.gender == other.gender
                && self.age == other.age
                && self.address == other.address {
            true
        } else {
            false
        }
    }
}
