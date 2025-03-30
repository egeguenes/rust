struct Potato {
    variety: String,
    weight: i32,
}

enum Flavor {
    Sparkling,
    Sweet,
    Fruity,
}

struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}

fn print_drink(drink: Drink) {
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

fn main() {
    let my_potato = Potato {
        variety: String::from("Sweet Potato"),
        weight: 13,
    };

    println!("{}\n\n", my_potato);

    let energy = Drink {
        flavor: Flavor::Sparkling,
        fluid_oz: 12.24,
    };

    print_drink(energy);

    let coord = (2, 3);
    let (x, y) = (2, 3);
    println!("\n\n({},{})", coord.0, coord.1);
    println!("({},{}\n\n", x, y);
}
