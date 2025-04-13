#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    let rect_tuple = (30, 50);

    let mock = 2;
    let rect_struct = Rectangle {
        width: dbg!(mock * 30),
        height: 50,
    };

    println!(
        "Lit:   The area of the rectangle is {} sqaure pixels!",
        area_lit(width1, height1)
    );
    println!(
        "Tuple: The area of the rectangle is {} sqaure pixels!",
        area_tuple(rect_tuple)
    );
    println!(
        "Struct: The are of the rectangle is {} square pixels!",
        area_struct(&rect_struct)
    );
    // Implementation of the structure -> Printing the area of it
    rect_struct.print_area();

    // Debug printing for the rect_structure
    println!("The rect is {:?}", rect_struct);

    // Better formatted debug printing
    println!("The rect is {:#?}", rect_struct);

    // A much better printing for it as we can see what happens and what we have input
    dbg!(&rect_struct);
    dbg!(area_lit(12, 12));
}

fn area_lit(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

impl Rectangle {
    fn print_area(&self) {
        println!(
            "\nStruct impl: The are of the rectangle is {} square pixels!",
            self.area()
        );
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}
