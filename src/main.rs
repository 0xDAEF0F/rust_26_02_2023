// 5.3 Defining an Enum

enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr2 {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    // can have named fields
    Move { x: i32, y: i32 },
    Write(String),
    // or tuple-like fields
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// very important concept!
enum Option<T> {
    None,
    Some(T),
}

fn main() {
    // instantiate enum
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    // instantiate struct
    let _home: IpAddr = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    // instantiate struct two
    let _loopback: IpAddr = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // instantiate struct three
    let _home2 = IpAddr2::V4(String::from("127.0.0.1"));
    let _loopback2 = IpAddr2::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    // option example
    let some_number = Some(5);
    let some_char = Some('a');

    let absent_number: Option<i32> = Option::None;

    let num = some_number.unwrap();
    println!("num + 5: {}", num + 5);
}
