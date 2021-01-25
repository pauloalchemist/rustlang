fn main() {
    let rect1 = (30, 60);
       
    println!(
        "A área do retângulo é {} pixels quadrado",
        area(rect1)
    );
}

fn area(dimensions:(u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}