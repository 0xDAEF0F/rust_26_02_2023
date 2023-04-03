// 7.1 Packages and Crates

// NOTES
// * A crate is the **smallest** amount of code that the Rust compiler considers at a time.
// * Crates *can* contain modules, and the modules *may* be defined in other files that get
//   compiled with the crate.
// * A crate can come in two forms:
//   1. Library crate: don't have a `main` fn and don't compile to an executable,
//      instead defined reusable functionality.
//   2. Binary crate: programs you can compile to an executable that you can run.
//      Each must have a function called `main`.
// * The *crate root* is a source file that the Rust compiler starts from and makes up the
//   root module of your crate.
// * A *package* is a bundle of one or more crates that provides a set of functionality.
//   They contain a `Cargo.toml` file that describes how to build those crates.
// * If a package contains `src/main.rs` and `src/lib.rs` it has a binary and a library
//   crate. Both packaged under the same name.
// * A package can have multiple binary crates by placing files in the `src/bin` directory.
fn main() {}
