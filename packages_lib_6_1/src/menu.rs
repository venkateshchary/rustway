#[derive(Debug)]
pub enum NonVeg {
    Mutton,
    Chicken,
    Crab,
    Egg,
}
#[derive(Debug)]
pub enum Veg {
    Dal,
    Sambar,
    LadyFinger,
    Panner,
}
#[derive(Debug)]
pub enum Curry {
    Veg(Veg),
    NonVeg(NonVeg),
}
#[derive(Debug)]
pub enum Starter {
    ChickenKebab,
    AluRoast,
    Chicken65,
    ChickenRoll,
    BabyCorn,
}
/*
   using struct inside a module
*/
#[derive(Debug)]
pub struct Lunch {
    pub customer_name: String,
    pub curry: Curry,
    water_bottle: String,
}

#[derive(Debug)]
pub struct Dinner {
    pub customer_name: String,
    phone_number: Option<i32>,
    pub curry: Curry,
    pub water_bottle: String,
    pub starter: Starter,
}

impl Lunch {
    pub fn new(curry: Curry, cust_name: String) -> Self {
        Self {
            curry,
            customer_name: cust_name,
            water_bottle: String::from("Kinley"),
        }
    }
}
