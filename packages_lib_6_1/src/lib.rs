
pub fn count_visited(){
    let count= 0;
    println!("as of now {:?}", count);
}

mod menu{
    #[derive(Debug)]
    pub enum NonVeg{
        Mutton,
        Chicken,
        Crab,
        Egg
    }
    #[derive(Debug)]
    pub enum Veg{
        Dal,
        Sambar,
        LadyFinger,
        Panner
    }
    #[derive(Debug)]
    pub enum Curry{
        Veg(Veg),
        NonVeg(NonVeg)
    }
    #[derive(Debug)]
    pub enum Starter{
        ChickenKebab,
        AluRoast,
        Chicken65,
        ChickenRoll,
        BabyCorn

    }
    /*
        using struct inside a module
     */
    #[derive(Debug)]
    pub struct Lunch{
        pub customer_name: String,
        pub curry: Curry,
        water_bottle: String,
    }

    #[derive(Debug)]
    pub struct Dinner{
        pub customer_name: String,
        phone_number: Option<i32>,
        pub curry: Curry,
        pub water_bottle: String,
        pub starter:Starter

    }

    impl Lunch{
        pub fn new(curry: Curry, cust_name: String) -> Self{
            Self{curry,
                customer_name:cust_name,
                water_bottle: String::from("Kinley")
            }
        }
    }
}
mod front_of_house { // snake_case (module)

    pub mod hosting{
        pub fn add_to_waiting_list(){
        }
        fn seat_at_table(){

        }
    }
    mod serving{
        use crate::menu::NonVeg::Chicken;

        fn take_order(){
            // here also you can use the super but the placement is
            // serving::front_of_house -> count_visited ->(i.e) ../../count_visited() -> super::super::count_visited()
            // we just need to climb up 2 steps
            // super::super::count_visited();
            // or another way we can directly use the crate, Because crate is the root module
            crate::count_visited();
            let customer1 = crate::menu::Lunch::new(
                crate::menu::Curry::NonVeg(crate::menu::NonVeg::Chicken),
                String::from("Venkatesh"),
            );

        }
        fn serve_order(){
            // After serving the order we need to take the payment

            take_payment();// we can call in this way when both are in same module space
        }
        fn take_payment(){

        }
    }
}

/*
    count_visited function is placed at same order of back_of_house module
 */

mod back_of_house {
    fn no_of_people_visited(){
        super::count_visited();
    }
}



/*
    entire module tree is rooted under the implicit module named crate
 */

pub fn eat_at_rest(){
    // absolute path
    crate::front_of_house::hosting::add_to_waiting_list(); // one way of calling the fn
    let mut candidate = back_of_house::Lunch::new(Veg(Dal));
    candidate.curry = Curry::NonVeg(Mutton); // becuase we mentioned curry as pub so we can modify it

    // if we try to modify the water bottle company it won't allow it is designed by the restaurent owner so we can't change it
    // candidate.water_bottle =  String::from("Oxyzen"); this won't work becuase of private field
    println!("{:?}", candidate);

    // relative path
    front_of_house::hosting::add_to_waiting_list(); //second way to call the fn

}