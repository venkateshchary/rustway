
### Crates(lib)
1. **Binary Crate**
   >Binary crates are programs you can compile to an executable that you can run, such as a command line program or a server. Each must have a function called main that defines what happens when the executable runs. All the crates we’ve created so far have been binary crates.
   
   > output is **Executable**

2. **Library Crate**

    >Library crates don’t have a main function, and they don’t compile to an executable. Instead, they define functionality intended to be shared with multiple projects.

   > output is **Reusable library(no main function)**

### Package
A package is a bundle of one or more crates that provides a set of functionality. A package contains a `Cargo.toml` file that describes how to build those crates.

- A package can contain as many binary crates as you like, but at most only one library crate.
- A package must contain at least one crate, whether that’s a library or binary crate.

##### How to create a package
```rust
$ cargo new rust-project
     Created binary (application) `rust-project` package
$ ls rust-project
Cargo.toml
src
$ ls rust-project/src
main.rs

```

##### 2 things keep in memory
- Cargo follows a convention that `src/main.rs` is the **crate root of a binary crate** with the same name as the package.

- Cargo knows that if the package directory contains `src/lib.rs`, the package contains a library crate with the same name as the package, and `src/lib.rs` is its crate root
  
> Cargo passes the crate root files to rustc to build the library or binary.

If a package contains src/main.rs and src/lib.rs, it has two crates: a binary and a library, both with the same name as the package.

A package can have multiple binary crates by placing files in the src/bin directory: each file will be a separate binary crate

**Let's learn about next topic** -> [control_scope](control_scope.md)


> **The contents of either of these two files(main.rs/lib.rs) form a module named crate**