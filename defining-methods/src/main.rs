#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, ohter: &Rectangle) -> bool {
        self.width > ohter.width && self.height > ohter.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 50,
        height: 70
    };
    let rect2 = Rectangle {
        width: 10,
        height: 60
    };
    let rect3 = Rectangle {
        width: 60,
        height: 80
    };

    println!("Area of the rectangle is {} square pixels", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}


