#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let length1 = 50;
    let width1 = 30;

    println!(
        "The area of the rectangle is {} square pixels.",
        area1(length1, width1)
    );

    let rect1 = (50, 30);
    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    );

    let rect2 = Rectangle { length: 50, width: 30 };
    println!("rect2 is {:#?}", rect2);
    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect2)
    );
}

fn area1(length: u32, width: u32) -> u32 {
    length * width
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1 
}

fn area3(rectangle: &Rectangle) -> u32 {
     rectangle.length * rectangle.width
}

