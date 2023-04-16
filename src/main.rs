// 8.0 Exercise

use std::{collections::HashMap, ops::Div};

fn main() {
    let my_vec = vec![1, 2, 3, 4];

    let median = median_value(&my_vec);

    println!("the median is: {}", median);
}

// Assumes vector is sorted
fn median_value(v: &Vec<i32>) -> i32 {
    if v.is_empty() {
        panic!("Error: empty vector")
    }

    let length = v.len();

    match length % 2 {
        0 => v[length.div(2)] + v[length.div(2) - 1],
        _ => v[length.div(2)],
    }
}
