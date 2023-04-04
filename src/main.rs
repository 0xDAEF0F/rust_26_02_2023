// 7.3 Paths for Referring to an Item in the Module Tree

// NOTES
// * we use a path in the same way we use a path when navigating a filesystem.
// * A path can take two forms:
//      * Absolute path: starting from crate root (crate name) or `crate` if current crate.
//      * Relative path: starts from current module. Uses `self`, `super, or an identifier in the current module.
// * Both absolute and relative paths are separated by double colons `::`.

// importing fn from lib
use rust_book_exercises::eat_at_restaurant;

fn main() {
    eat_at_restaurant();

    let _breakfast = rust_book_exercises::front_of_house::Breakfast {
        toast: String::from("toast"),
        seasonal_fruit: String::from("mango"),
    };
}
