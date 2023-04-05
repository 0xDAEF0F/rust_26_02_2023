// 7.4 Bringing Paths into Scope with the `use` Keyword

// NOTES
// * If there is a name collision with an item in scope
//   you can use the `as` keyword to disambiguate the name.
use rand::Rng;
use rust_book_exercises::inc;
use rust_book_exercises::module_a::EnumA;
use rust_book_exercises::one;
use std::collections::HashMap as MyHashMap;
#[allow(unused_imports)]
use std::{cmp::Ordering, io}; // can save up vertical space
                              // use std::*; // can use glob operator to bring all into scope

fn main() {
    let one: u32 = one();
    let two: u32 = inc(one);

    println!("one: {one}, two: {two}");

    // --
    let _void: EnumA = EnumA::Void;
    let _unit: EnumA = EnumA::Unit;

    // --
    let mut map: MyHashMap<u8, u8> = MyHashMap::new();
    map.insert(0, 1);

    // --
    let _secret_number = rand::thread_rng().gen_range(1..=100);
}
