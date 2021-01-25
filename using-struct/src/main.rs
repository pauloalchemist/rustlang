#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let rect1 = Rectangle{
        width: 30,
        height: 60
    };

    println!("rect1 is {:#?}", rect1);
       
    println!(
        "A área do retângulo é {} pixels quadrado",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}