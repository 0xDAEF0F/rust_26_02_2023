// 8.2 Storing UTF-8 Encoded Text with Strings

// Notes:
// * Discuss strings in the context of collections of bytes
// * `String` is a growable, mutable, owned, UTF-8 encoded string type.
// * Both `String` and string slices are UTF-8 encoded.
// * Strings are implemented as a wrapper around a vector of bytes with
//   extra guarantees, restrictions, and capabilities.
// * `to_string` method is available on any type that implements the `Display` trait.
fn main() {
    // 1. Creating a String
    let _s0: String = "initial contents".to_string();
    let _s1: String = String::from("initial contents");

    // 2. Updating a String
    let mut _s2: String = "foo".to_string();
    // push_str does not take ownership of it's argument
    _s2.push_str("bar");
    // `push` takes a single char and adds to the String
    _s2.push('!'); // => foobar!

    // 3. Concatenation with the `+` Operator or the `format!` macro
    let _s3 = String::from("Hello, ");
    let _s4 = "world!".to_string();
    let _s5 = _s3 + &_s4; // s3 has moved to s5 and can't be used further
                          // `fn add(self, s: &str) -> String`
                          // &String can be coerced into &str
    let _s6 = String::from("tic");
    let _s7 = String::from("tac");
    let _s8 = String::from("toe");
    // this does not take ownership of any of the values and returns a String with the contents.
    let _s9 = format!("{_s6}-{_s7}-{_s8}");

    // 4. Indexing into Strings
    // this code will not compile. you can't access individual chars by referencing by index
    // let s1 = String::from("hello");
    // let h = s1[0];

    // 5. Internal Representation
    // * The reason that you can't access `&str[i]` directly is because of the fact
    //   that some UTF-8 chars are more than **one byte** in size.
    // * Three ways to look at strings from Rust's perspective: bytes, scalar values, and grapheme clusters.
    // * You can do instead: `&str[n..m]` to be more specific.

    // 6. Methods for Iterating Over Strings
    let weird_string = "Зд".to_string();
    // Option A
    for c in weird_string.chars() {}
    // Option B
    for b in weird_string.bytes() {}
    // Remember: valid unicode scalar values may be more than 1 byte

    // 7. Summary
    // * `contains` and `replace` check it out.
}
