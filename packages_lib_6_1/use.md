### Bringing module into scope with ``use``

Adding `use` and a path in a scope is similar to creating a symbolic link in the filesystem

> **Use imports are NOT global**

🔴 Key rule (VERY important)

> **use imports are NOT global.**
> 
> **They are scoped to the module they appear in.**

##### Without using the use statement
```rust

mod front_house{
    
    mod hosting{
        
        pub fn add_to_waitlist(){
            
        }
    }
}

pub fn eat_at_restaurant(){
    crate::front_house::hosting::add_to_waitlist();
}
```

#### with using ``use`` statement

when you use the ``use`` statement the submodule will be visible to same as parent module

```scss
crate
├── use hosting   ← visible here
├── eat_at_restaurant() ✅
└── front_house
```

```rust
use crate::front_house::hosting;

mod front_house{
    
    pub mod hosting{
        
        pub fn add_to_waitlist(){
            
        }
    }
}

pub fn eat_at_restaurant(){
    hosting::add_to_waitlist();
}
```
#### Why below one gives an error?
```rust
use crate::front_house::hosting;

mod front_house{
    
    pub mod hosting{
        
        pub fn add_to_waitlist(){
            
        }
    }
}

mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
```
eat_at_restaurant is place under the customer module so it is not in the same place hosting
due to this it will give scope error

```scss
crate
├── use hosting        ← ONLY here
├── front_house
├── customer
│   └── eat_at_restaurant ❌

```
####  How to correct

Use the absolute path inside the `customer`
```rust
mod customer {
    pub fn eat_at_restaurant() {
        crate::front_house::hosting::add_to_waitlist();
    }
}

```

#### Why can't we use ``use`` statement upto add_to_waitlist()

```rust
//instead of 
use crate::front_of_house::hosting;

// why can't we use path upto add_to_waitlist;
use crate::front_of_house::hosting::add_to_waitlist;
/*
so that we can directly call using the add_to_waitlist();
instead of hosting::add_to_waitlist()
 */

```
##### Answer:
Specifying the parent module when calling the function makes it clear that the function isn’t locally defined while still minimizing repetition of the full path

### We could provide new names using ``as`` Keyword
```rust
use crate::front_house::hosting as FrontHouseHosting;

mod front_house{
    
    pub mod hosting{
        
        pub fn add_to_waitlist(){
            
        }
    }
}

pub fn eat_at_restaurant(){
    FrontHouseHosting::add_to_waitlist();
}
```

## pub use vs use
### Real-life analogy
`use`

You borrow a book and read it inside your room.

`pub use`

You put the book on a public shelf with your name on it so others can read it from you.

### What it does
1. Makes it **publicly visible** as part of your module's API
```rust
pub use crate::front_house::hosting;

mod customer {
    fn eat_at_restaurant() {
        hosting::add_to_waitlist(); // here also you can use
    }
}
```

```text
visible here ✅
visible to child modules ✅
visible to external crates ✅

```

### Re-export

Using pub keyword we can re-export the modules
```rust
pub use crate::menu::Lunch; // Lunch is a Struct inside a menu module


mod front_of_house {

    pub mod hosting{
        fn take_order(){
            let customer1 = crate::Lunch::new(
                crate::menu::Curry::NonVeg(crate::menu::NonVeg::Chicken),
                String::from("Venkatesh"),
            );

        }
    }
}
```
I wanted to use the Lunch struct inside the front_house::hosting::take_order

If you are exported module in below way
```rust
use crate::front_of_house::hosting;
use crate::menu::Lunch;

mod front_of_house {
    pub mod hosting {
        fn take_order() {
            let customer1 = Lunch::new(...); // ❌ ERROR
        }
    }
}
```
##### Why?
Because front_of_house::hosting is a sibling module, not a child of the crate root.
```scss
crate
├── use crate::menu::Lunch   ← visible ONLY here
├── mod menu
├── mod front_of_house
│   └── mod hosting
│       └── take_order()     ← ❌ cannot see Lunch
└── fn eat_at_restaurant()   ← ✅ can see Lunch

```
✅ Option 1: Add use inside the module where you need it
```rust
mod front_of_house {
    pub mod hosting {
        use crate::menu::Lunch;
        use crate::menu::{Curry, NonVeg};

        pub fn take_order() {
            let customer1 = Lunch::new(
                Curry::NonVeg(NonVeg::Chicken),
                String::from("Venkatesh"),
            );
        }
    }
}

```
✅ Option 2: Use full paths (always works)
```rust

let customer1 = crate::menu::Lunch::new(
    crate::menu::Curry::NonVeg(crate::menu::NonVeg::Chicken),
    String::from("Venkatesh"),
);

```
✅ Option 3: Re-export

at crate root
```rust
pub use crate::menu::Lunch;
```

Then hosting
```rust
use crate::Lunch;
```

### Combine imports

```rust
use std::cmp::Ordering;
use std::io;
// Becomes
use std::{cmp::Ordering,io};
```