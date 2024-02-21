// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum Color {
    Red,
    Green
}

impl Color {
    fn print(&self) {
        match self {
            Color::Red => println!("red"),
            Color::Green => println!("green")
        }
    }
}

struct Dimensions {
    width: i32,
    height: i32,
    depth: i32
}

impl Dimensions {
    fn print(&self) {
        println!("{:?}x{:?}x{:?}", self.width, self.height, self.depth)
    }
}

struct ShippingBox {
    dimensions: Dimensions,
    weight: i32,
    color: Color
}

impl ShippingBox {
    fn create(color: Color, dimensions: Dimensions, weight: i32) -> Self {
        ShippingBox {
            color, dimensions, weight
        }
    }

    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("{:?}", self.weight);
    }
}

fn main() {
    let shipping_box = ShippingBox::create(
        Color::Red, 
        Dimensions { width: 20, height: 10, depth: 5 }, 
        32
    );
    
    shipping_box.print();
}
