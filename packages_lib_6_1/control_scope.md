
`use` keyword that brings path into scope

``pub`` keyword to make item public

> crate root files(src/lib.rs & src/main.rs)

say you declare a `garden` module with `mod garden`;

the compiler will look for the module's code in these places

- In the file src/garden.rs
- In the file src/garden/mod.rs

#### Declaring sub modules

**In any file other than crate root** you declare the submodules

you might declare mod vegetables; in src/garden.rs

- Inline, directly following mod vegetables, within curly brackets instead of the semicolon
- In the file src/garden/vegetables.rs
- In the file src/garden/vegetables/mod.rs

#### paths to code in modules
An `Asparagus` type in the garden vegetables module would be found at `crate::garden::vegetables::Asparagus`.

### Public vs Private
Code within a module is private from its parent modules by default. 
- To make a module public, declare it with `pub mod` instead of `mod`
- To make items within a public module public as well, use pub before their declarations.
