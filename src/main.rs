// 8.3 Storing Keys with Associated Values in Hash Maps

use std::collections::HashMap;

// Notes:
// * Useful to look up data by key
// * Store their data on the heap (like Vectors)
// * Keys *and* values must have the same type
// * We can insert references to a hashmap but the values that the ref point to must be valid
//   for at least as long as the hash map is valid.
// * Each key can *only* have one value associated at a time.
// * Iterating over a hash map happens in arbitrary order.
// * `SipHash` used as hashing algorithm.
fn main() {
    let mut scores = HashMap::new();

    scores.insert("Blue".to_string(), 10);
    scores.insert("Yellow".to_string(), 50);

    // copied copies the Option value and unwrap just unwraps the option and if none sets to 0
    let score = scores.get(&String::from("Blue")).copied().unwrap_or(0);

    // iterate over each k/v pair
    for (k, v) in &scores {}

    // 2. Hash Maps and Ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // can no longer use `field_name` or `field_value` because ownership moved to the map

    // 3. Overwriting a value
    overwriting_a_value();

    // 4. Adding a Key and Value Only if a Key Isn't Present
    writing_only_if_non_existent_key();

    // 5. Updating a Value Based on the Old Value
    updating_based_old_value();
}

fn overwriting_a_value() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);
}

fn writing_only_if_non_existent_key() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    let ent = scores.entry(String::from("Yellow")).or_insert(50);
    *ent += 1;
    let ent2 = scores.entry(String::from("Blue")).or_insert(50);
    *ent2 += 1;

    println!("{:?}", scores);
}

fn updating_based_old_value() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
