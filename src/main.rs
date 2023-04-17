// 9.1 Unrecoverable Error with `panic!`

// Notes
// * When reading from an out of bounds array, rust panics automatically.
// * Whenever there is an unrecoverable error, there is no choice but to panic.
fn main() {
    // automatic panicking
    let v = vec![1, 2, 3];

    v[99];

    // manual panicking
    panic!("crash and burn!");
}
