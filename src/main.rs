// 7.2 Defining Modules to Control Scope and Privacy

// NOTES
// * `use` brings path into scope.
// * `pub` make items public.
// DECLARING MODULES
// * `mod` to declare a module. The compiler will look for `mod garden;` in:
//    * Inline, i.e., `mod garden {}`
//    * `src/garden.rs`
//    * `src/garden/mod.rs`
// DECLARING SUBMODULES
// * Declare any other file other than *crate root*.
//   * ex: `mod vegetables;` in `src/garden.rs`
//   * Inline, i.e., `mod vegetables {}`
//   * `src/garden/vegetables/rs`
//   * `src/garden/vegetables/mod.rs`
// PATH TO CODE IN MODULES
// * Once part of your crate, `Asparagus` -> `crate::garden::vegetables::Asparagus`
// PRIVATE VS PUBLIC
// * private by default.
// * to make it public: `pub mod`.
// THE `use` KEYWORD
// * allows to reduce the repetition of long paths.

// bring into scope (shorthand)
use crate::garden::vegetables::Asparagus;

// tell compiler to include `src/garden.rs`
pub mod garden;
fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
