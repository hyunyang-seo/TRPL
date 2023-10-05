#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }

    fn area(&self) -> u32 {
        self.length * self.width
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length >= other.length && self.width >= other.width
    }
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };
    let rect2 = Rectangle { length: 40, width: 10 };
    let rect3 = Rectangle { length: 45, width: 60 };
    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     rect1.area()
    // );
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect2.can_hold(&rect3));
    let sq = Rectangle::square(3);
    println!("Square {:?} area is {}", sq, sq.area());
}
