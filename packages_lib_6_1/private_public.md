In Rust, all items (functions, methods, structs, enums, modules, and constants) are private to parent modules by default

If you want to make an item like a function or struct private, you put it in a module.

Rust does give you the option to expose inner parts of child modules’ code to outer ancestor modules by using the pub keyword to make an item public.

- Adding the pub keyword in front of mod hosting makes the module public. With this change, if we can access front_of_house, we can access hosting. But the contents of hosting are still private;

```rust
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}
```