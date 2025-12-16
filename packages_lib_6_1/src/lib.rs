
fn count_visited(){
    let count= 0;
    println!("as of now {:?}", count);
}
mod front_of_house { // snake_case (module)

    pub mod hosting{
        pub fn add_to_waiting_list(){
            crate::count_visited();
        }
        fn seat_at_table(){

        }
    }
    mod serving{
        fn take_order(){
            // here also you can use the super but the placement is
            // serving::front_of_house -> count_visited ->(i.e) ../../count_visited() -> super::super::count_visited()
            // we just need to climb up 2 steps
            super::super::count_visited();
            // or another way we can directly use the crate, Because crate is the root module
            crate::count_visited();

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

    // relative path
    front_of_house::hosting::add_to_waiting_list(); //second way to call the fn

}