// 5.3 Method Syntax
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// can have another impl block (no reason to do so but ok)
impl Rectangle {
    fn area_gt_100(&self) -> bool {
        self.area() > 100
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of rect1 is: {}", rect1.area());

    println!("The width is greater than 0: {}", rect1.width());

    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };
    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));

    let square = Rectangle::square(10);
    println!("rect1 can hold square: {}", rect1.can_hold(&square));

    println!("square area is gt 100: {}", square.area_gt_100());
}
