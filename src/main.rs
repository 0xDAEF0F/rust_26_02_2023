// 4 Understanding Ownership
// 4.1 What is Ownership?
fn main() {
    // example_0();
    // example_1();
    // example_2();
    // example_3();
    // example_4();
    // example_5();
    example_6();
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn example_0() {
    let s = "hello";
    {
        // another scope
        let s = "another hello";
    }
    // This String type is allocated on the heap
    // unknown at compile time and growable
    {
        let mut s2 = String::from("hello");
        s2.push_str(", world!");
        // Once the variable goes out of scope, the memory is freed
        println!("{}", s2);
    }
}

#[allow(dead_code)]
fn example_1() {
    let s1 = String::from("hello");
    let s2 = s1;
    // when we assign s1 to s2, the String data is copied, meaning
    // we copy the pointer, the length, and the capacity that are
    // on the stack

    // This will error because s1 is no longer valid
    // println!("{}, world!", s1);

    // OK!
    println!("{}, world!", s2);
}

#[allow(dead_code)]
fn example_2() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    // This will work because we are explicitly calling clone
    // which will copy the heap data
    println!("s1 = {}, s2 = {}", s1, s2);
}

#[allow(dead_code)]
fn example_3() {
    // This is fine!
    let x = 5;
    let y = x;

    // types such as integers that have a known
    // size at compile time are stored entirely
    // on the stack.
    println!("x = {}, y = {}", x, y);
}

#[allow(dead_code)]
fn example_4() {
    let s = String::from("hello");
    takes_ownership(s);
    // This will error because s is no longer valid
    // println!("{}", s);

    let x = 5;
    makes_copy(x);
    // This will work because x is an integer
    // which is stored on the stack
    println!("{}", x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

#[allow(dead_code)]
fn example_5() {
    let s1 = gives_ownership();
    println!("{}", s1);
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    // This will error because s2 is no longer valid
    // println!("{}", s2);

    println!("{}", s3);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    return some_string;
}

fn takes_and_gives_back(a_string: String) -> String {
    return a_string;
}

#[allow(dead_code)]
fn example_6() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    return (s, length);
}
