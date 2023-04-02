// 6.2 The match control flow construct

// CHALLENGE
// * construct a match statement
// * extract value from the type of that match (try Option)
fn main() {
    let other_book = Book::Other;
    let math_book_easy = Book::Math { iq: 100 };
    let math_book_hard = Book::Math { iq: 150 };

    let book_arr = [&other_book, &math_book_easy, &math_book_hard];

    let mut i = 0;
    while i < 3 {
        let is_hard = is_book_too_hard(&book_arr[i]);
        println!("book {i} is: {is_hard}");
        i += 1;
    }
}

enum Book {
    Math { iq: u8 },
    Other,
}

fn is_book_too_hard(book: &Book) -> bool {
    match book {
        Book::Math { iq } => iq > &120,
        _ => false,
    }
}
