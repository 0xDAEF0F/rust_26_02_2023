// 8.1 Storing Lists of Values with Vectors

// Notes
// * Data collections point to is stored on the heap. (unknown compile time + can grow dynamically)
// * `Vector` allows you to store a variable number of values next to each other.
// * `String` is a collection of characters.
// * `Hash` map allows you to associate a value with a particular key.
fn main() {
    // Option 1
    let _v: Vec<i32> = Vec::new();

    // Option 2: Macro
    let mut v2: Vec<i32> = vec![1, 2, 3];

    // To add elems (notice the mut keyword):
    v2.push(4);

    // -----------------------------------------------

    // Read Elems From Vector
    // Method 1: if you reference a non-existent element
    //           program will panic.
    let zeroth: &i32 = &v2[0];
    println!("The zeroth elem is: {}", zeroth);

    // Method 2: safer way to read an element you are not
    //           sure it exists.
    let zeroth_v2: Option<&i32> = v2.get(0);
    match zeroth_v2 {
        Some(zeroth) => println!("The zeroth elem is {}", zeroth),
        None => println!("There is no zeroth element"),
    }

    // -----------------------------------------------

    let my_vector: Vec<i32> = vec![1];
    let _first: &i32 = &my_vector[0];

    // Will not compile: can not insert an element into a vector
    // while having a reference to it.
    // my_vector.push(1);

    // -----------------------------------------------

    // Iterating over values in a vector
    let vector: Vec<i32> = vec![100, 32, 57];
    for i in &vector {
        println!("{i}")
    }

    // Iterate over mutable references to vector
    let mut vector_two: Vec<i32> = vec![100, 32, 57];
    for i in &mut vector_two {
        // need to dereference the value
        *i *= 2;
        println!("{i}");
    }

    // -----------------------------------------------

    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // Note how under an enum umbrella you can store different
    // types, although technically they are the same type.
    {
        let _row: Vec<SpreadSheetCell> = vec![
            SpreadSheetCell::Int(1),
            SpreadSheetCell::Text(String::from("blue")),
            SpreadSheetCell::Float(10.12),
        ];
    } // after this scope the vector and it's items are dropped
}
