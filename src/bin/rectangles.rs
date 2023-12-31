#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn get_area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        (self.height >= rectangle.height && self.width >= rectangle.width)
            || (self.height >= rectangle.width && self.width >= rectangle.height)
    }

    fn create_square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: dbg!(30 * 2),
        height: 50,
    };

    // Using methods
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.get_area()
    );
    // Using methods
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    // Ugly printing
    println!("rect1 is {:?}", rect1);
    // Pretty printing
    println!("rect1 is {:#?}", rect1);
    // Pretty printing with debug information. Also, takes ownership
    dbg!(&rect1);

    // Method with more parameters
    let rect2 = Rectangle {
        width: 60,
        height: 50,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    // Associated function
    println!(
        "This is a square rectangle: {:#?}",
        Rectangle::create_square(3)
    )
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
