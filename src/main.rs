fn main() {
    // cond_loop_w_while();
    // loop_through_collection_with_for();
    // loop_with_for_in_loop();
    loop_with_for_in_loop_with_range();
}

#[allow(dead_code)]
fn cond_loop_w_while() {
    let mut x = 3;
    while x != 0 {
        println!("{x}!");
        x -= 1;
    }
    println!("LIFTOFF!!!");
}

#[allow(dead_code)]
fn loop_through_collection_with_for() {
    let i_collection = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", i_collection[index]);
        index += 1;
    }
}

#[allow(dead_code)]
fn loop_with_for_in_loop() {
    let i_collection = [10, 20, 30, 40, 50];
    for element in i_collection {
        println!("the value is: {element}");
    }
}

#[allow(dead_code)]
fn loop_with_for_in_loop_with_range() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}