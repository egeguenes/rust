struct Potato {
    variety: String,
    weight: i32,
}

struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}

struct GroceryItem {
    id: i32,
    quantity: i32,
}

enum Flavor {
    Sparkling,
    Sweet,
    Fruity,
}

enum UniMember {
    Student,
    Professor,
}

fn print_drink(drink: &Drink) {
    match drink.flavor {
        Flavor::Sparkling => println!("Flavor: Sparkling"),
        Flavor::Sweet => println!("Flavor: Sweet"),
        Flavor::Fruity => println!("Flavor: Fruity"),
    }

    println!("Oz: {}", drink.fluid_oz);
}

impl std::fmt::Display for Potato {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "A {} potato weighs {:.1} kg", self.variety, self.weight);
    }
}

fn introduce(mem: UniMember) {
    match mem {
        UniMember::Student => println!("I am a student!"),
        UniMember::Professor => println!("I am a professor!"),
    }
    // at the function exit the mem is deletedas the introduce function gets the ownership of the object from main function
}

fn introduce_borrow(mem: &UniMember) {
    match mem {
        UniMember::Student => println!("I am a student in the borrow function!"),
        UniMember::Professor => println!("I am a professor in the borrow function!"),
    }
}

fn display_id(item: &GroceryItem) {
    println!("The id of the item is {:?}", item.id);
}

fn display_quantity(item: &GroceryItem) {
    println!("The quantity of the item is {:?}", item.quantity);
}

impl Drink {
    fn sweeten(self) -> Self {
        Self {
            flavor: Flavor::Sweet,
            fluid_oz: self.fluid_oz,
        }
    }

    fn display_drink_impl(&self) {
        print_drink(&self);
    }
}

fn main() {
    let my_potato = Potato {
        variety: String::from("Sweet Potato"),
        weight: 13,
    };

    println!("{}\n\n", my_potato);

    let _flavors = [Flavor::Fruity, Flavor::Sparkling, Flavor::Sweet];
    let energy = Drink {
        flavor: Flavor::Sparkling,
        fluid_oz: 12.24,
    };

    print_drink(&energy);
    energy.display_drink_impl();
    let energy = Drink::sweeten(energy);
    energy.display_drink_impl();

    let coord = (2, 3);
    let (x, y) = (2, 3);
    println!("\n\n({},{})", coord.0, coord.1);
    println!("({},{}\n\n", x, y);

    let three = 3;
    let magic_number = match three {
        3 => "Magic number is three",
        4 => "Magic number is four",
        _ => "Magic number is neither three nor four",
    };
    /*
        let is_three = three == 3;
    */
    println!("Magic number should be 3\n{}\n", magic_number);

    let student = UniMember::Student;
    introduce(student);
    //introduce(student); Cant call this one, as the first call gets the ownership and the member was deleted in the first call.

    let professor = UniMember::Professor;
    introduce_borrow(&professor);
    introduce_borrow(&professor);

    let apples: GroceryItem = GroceryItem {
        id: 1,
        quantity: 13,
    };
    display_id(&apples);
    display_quantity(&apples);
}

fn myfunc(num: &mut i32) -> i32 {
    *num = *num + 1;
    num
}
