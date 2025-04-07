const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = x + 1;
    println!("The value of x is {x}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("{spaces}");

    let parsing_u32: u32 = "132".parse().expect("Not a num!");
    let parsing_i32: i32 = "132".parse().expect("Not a num!");
    // let error = "132".parse().expect("Not a num!");

    let tup = (12, 13, 30.0);
    let tup2: (i32, i32, f64) = (12, 13, 30.0);
    let (x, y, z) = tup;
    println!("The values are {x}, {y}, {z}.");
}
