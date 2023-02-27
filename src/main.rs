// 4.3 The Slice Type
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
    let mut s = String::from("hello world");

    let fw = first_word(&s); // 5

    s.clear(); // empties s: ""

    // fw is still 5, but value of s is now ""
    println!("first word ended at index: {fw}");
    println!("string is now: \"{s}\"");
}

// b' ' is a byte literal
// bytes.iter() returns an iterator over the bytes of the string
// enumerate() wraps the result of bytes.iter() and returns each element as part of a tuple
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    return s.len();
}

#[allow(dead_code)]
fn example_1() {
    // enter: String Slices
    let mut s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("the first word is: \"{hello}\"");
    println!("the second word is: \"{world}\"");

    s.clear();

    // this will panick because index 5 does not exist
    let not_hello = &s[0..5];

    println!("not_hello: {not_hello}");
}

#[allow(dead_code)]
fn example_2() {
    let s = String::from("hello");

    // Equivalent
    let he = &s[0..2];
    let he2 = &s[..2];
    assert_eq!(he, he2);

    // Equivalent
    let len = s.len();
    let lo = &s[3..len];
    let lo2 = &s[3..];
    assert_eq!(lo, lo2);

    // Equivalent
    let all = &s[0..len];
    let all2 = &s[..];
    assert_eq!(all, all2);

    let mut s2 = String::from("hello");
    println!("s2 before: \"{s2}\"");
    s2.clear();
    println!("s2 after: \"{s2}\"");
}

#[allow(dead_code)]
fn example_3() {
    let s = String::from("hello ser");
    let fw = fw(&s);
    println!("fw is: {fw}");

    let sw = second_word(&s);
    println!("sw is: {sw}");
}

fn fw(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i + 1..];
        }
    }

    &s[..]
}

#[allow(dead_code)]
#[allow(unused_mut)]
fn example_4() {
    let mut s = String::from("hello world");

    let fw = fw(&s);

    // If uncommented
    // Will error out: cannot borrow `s` as mutable because it is also borrowed as immutable
    // when trying to print fw
    // s.clear();

    println!("the first word is: {fw}");
    println!("the string is: {s}");
}

#[allow(dead_code)]
fn example_5() {
    let s = "Hello, world!";
    let first_word = fw(&s);
    assert_eq!(first_word, "Hello,");
    println!("first word: {first_word}");
    println!("entire string: {s}");
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn example_6() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = fw(&my_string[0..6]);
    let word = fw(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = fw(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = fw(&my_string_literal[0..6]);
    let word = fw(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = fw(my_string_literal);
}

#[allow(dead_code)]
fn example_7() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}
