
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


### Declaring a module that don't have submodules

```rust
src
├── garden.rs
└── main.rs
```

### Declaring a module that contains the submodules

```rust
backyard
└── src
    ├── garden          <-- folder for garden's submodules
    │   └── vegetables.rs
    ├── garden.rs       <-- declares module `garden`
    └── main.rs
```

### Declaring a module that contains 2 inner submodules
```rust
src
├── main.rs
├── garden.rs
└── garden
    ├── vegetables.rs
    └── vegetables
        └── leafy.rs
```
> Note:
> If you observe carefully if module contains submodules then we need to define a folder with same name of module
> inside of it we need to add the submodule