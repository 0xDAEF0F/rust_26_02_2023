use std::option::Option;
// 6.3 Concise Control Flow with if let

// CHALLENGE
// * Do an example of an if let, else situation
fn main() {
    let value_one = Some(100);
    let value_two: Option<i32> = None;

    if let Some(value) = value_one {
        println!("the value is: {value}");
    } else {
        println!("this is a none")
    }

    let digested_value_two = extract_value(value_two);
    println!("value two's value is: {digested_value_two}");

    let five = 5;
    let ten = double(five);
    println!("just doubled {} to {}", five, ten);
}

fn extract_value(val: Option<i32>) -> i32 {
    if let Some(num) = val {
        return num;
    } else {
        return 0;
    };
}

fn double(num: i32) -> i32 {
    num * 2
}
