// 5.1 Defining and Instantiating Structs
fn main() {
    // example_0();
    // example_1();
    // example_2();
    // example_3();
    example_4();
}

#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn example_0() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // to read
    let user1_email = user1.email;
    println!("user1_email: {}", user1_email);

    // to write
    // entire instance *MUST* be mutable
    user1.email = String::from("another_email@example.com");
    println!("new user1_email: {}", user1.email);

    // create new user
    let user2 = build_user(String::from("user2@xyz.xyz"), String::from("user2"));
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn example_1() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // valid to reference other struct
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@email.com"),
        sign_in_count: user1.sign_in_count,
    };

    // valid to destructure
    // can no longer user user1
    let user3 = User {
        email: String::from("user3@email.com"),
        ..user2
    };

    let user1_active = user1.active;
    println!("is user1 active? {}", user1_active);

    // can still reference user1 email. but not as a whole?
    let user1_email = user1.email;
    println!("user1_email: {}", user1_email);
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[allow(dead_code)]
#[allow(unused_variables)]
fn example_2() {
    // tuple structs
    // they are different types, even if their fields
    // are the same type
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

// unit-like struct
struct AlwaysEqual;

#[allow(dead_code)]
#[allow(unused_variables)]
fn example_3() {
    let subject = AlwaysEqual;
}

#[allow(dead_code)]
fn example_4() {
    // error: missing lifetime specifier
    // struct User {
    //     active: bool,
    //     username: &str,
    //     email: &str,
    //     sign_in_count: u64,
    // }

    // let user1 = User {
    //     active: true,
    //     username: "someusername123",
    //     email: "someone@example.com",
    //     sign_in_count: 1,
    // };
}
