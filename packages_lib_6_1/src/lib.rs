mod front_of_house { // snake_case (module)
    pub mod hosting{
        pub fn add_to_waiting_list(){

        }
        fn seat_at_table(){

        }
    }
    mod serving{
        fn take_order(){

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
    entire module tree is rooted under the implicit module named crate
 */

pub fn eat_at_rest(){
    // absolute path
    crate::front_of_house::hosting::add_to_waiting_list(); // one way of calling the fn

    // relative path
    front_of_house::hosting::add_to_waiting_list(); //second way to call the fn

}