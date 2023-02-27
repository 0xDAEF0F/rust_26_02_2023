// 4.2 References and Borrowing
fn main() {
    // example_0();
    // example_1();
    // example_2();
    // example_3();
    // example_4();
    // example_5();
    // example_6();
    example_7();
}

#[allow(dead_code)]
fn example_0() {
    let s1 = String::from("hello, ser");
    // this pattern is called borrowing
    let len = calculate_length(&s1);
    // we can pass a reference to calculate_length
    // and s1 will still be valid and owned by this scope
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    // a reference allows us to use a value without taking ownership
    s.len()

    // NOTE: s.push_str(", world"); // this will not work because we do not have ownership
} // s goes out of scope, but since it does not have ownership of what it refers to, it is not dropped.

#[allow(dead_code)]
fn example_1() {
    // declare a variable that is mutable
    let mut s = String::from("hello ser");
    change(&mut s); // create mutable reference

    println!("The string is now: {}", s);
}

// This &mut makes very clear that the value will mutate
fn change(some_string: &mut String) {
    some_string.push_str(", another string");
}

#[allow(dead_code)]
fn example_2() {
    // If you have a mutable reference to a value,
    // you can have no other references to that value.
    let mut s = String::from("mutable string");

    // Fine!
    let r1 = &mut s;
    r1.push('.');

    // Not Fine!
    // let r2 = &mut s;

    println!("r1 is {}", r1);
}

#[allow(dead_code)]
fn example_3() {
    let mut s = String::from("mutable string");

    {
        let r1 = &mut s;
        r1.push('.');
    } // r1 gets dropped here

    #[allow(unused_variables)]
    let r3 = &s; // no problem
    let r2 = &mut s; // no problem

    // BIG PROBLEM
    // let r3 = &mut s;

    println!("r2 is {r2}");
}

#[allow(dead_code)]
#[allow(unused_mut)]
fn example_4() {
    // You can have multiple immutable references to a value
    // but you can only have one mutable reference to a value
    let mut s = String::from("mutable string");

    let r1 = &s; // no problem
    let r2 = &s; // no problem

    // BIG PROBLEM
    // let r3 = &mut s;

    println!("{}, {}", r1, r2);
    // println!("{}, {}, and {}", r1, r2, r3);
}

#[allow(dead_code)]
fn example_5() {
    // let mut s = String::from("mutable string");

    // let r1 = &s;
    // let r2 = &mut s;

    // ERROR
    // println!("{}, {}", r1, r2);
}

#[allow(dead_code)]
fn example_6() {
    let mut s = String::from("hello");
    println!("s is {}", s);

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    s.push_str(", world!");

    let r3 = &mut s;

    r3.push('!');

    println!("{}", r3);
    println!("{}", s);
}

#[allow(dead_code)]
fn example_7() {
    // let reference_to_nothing = dangle();
    let h = no_dangle();
    println!("h is {}", h);
}

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

fn no_dangle() -> String {
    return String::from("hello");
}

// Rules of References
// 1. At any given time, you can have either one mutable reference or any number of immutable references.
// 2. References must always be valid.
