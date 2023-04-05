// specifying that we wan't to bring the module front_of_house.
// it will look in a file with that name.
mod front_of_house;

// bring `add_to_waitlist` to namespace.
use front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    // option 1: provide absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // option 2: namespace option
    add_to_waitlist();

    let _two: u32 = front_of_house::hosting::inc(1);
}
