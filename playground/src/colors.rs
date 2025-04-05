enum Color {
    Red,
    Green,
    Blue,
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

struct Box {
    color: Color,
    weight: f64,
    dimensions: Dimensions,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Red => println!("Color is Red"),
            Color::Green => println!("Color is Green"),
            Color::Blue => println!("Color is Blue"),
        }
    }
}

impl Dimensions {
    fn print(&self) {
        println!("Width: {:?}", self.width);
        println!("Height: {:?}", self.height);
        println!("Depth: {:?}", self.depth);
    }
}

impl Box {
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            weight,
            color,
            dimensions,
        }
    }

    fn print(&self) {
        self.color.print();
        println!("Weight: {:?}", self.weight);
        self.dimensions.print();
    }
}

fn main() {
    let my_box = Box {
        weight: 1222.3,
        color: Color::Red,
        dimensions: Dimensions {
            width: 12.0,
            height: 23.0,
            depth: 34.0,
        },
    };

    let color_red = Color::Red;
    let weight = 1222.3;
    let dimensions = Dimensions {
        width: 12.0,
        height: 23.0,
        depth: 34.0,
    };
    let my_box2 = Box::new(weight, color_red, dimensions);

    my_box.print();
    println!();
    my_box2.print();
}
