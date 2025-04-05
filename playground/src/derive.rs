#[derive(Debug)]
enum MyEnum {
    Option1,
    Option2,
    Option3,
}

#[derive(Debug)]
struct MyStruct {
    rand_enum: MyEnum,
    number: i32,
}

fn print_option(a_struct: &MyStruct) {
    match a_struct.rand_enum {
        MyEnum::Option1 => println!("Option is 1"),
        MyEnum::Option2 => println!("Option is 2"),
        MyEnum::Option3 => println!("Option is 3"),
    }
}

fn main() {
    let a_struct = MyStruct {
        rand_enum: MyEnum::Option1,
        number: 1,
    };

    println!("{:?}", a_struct.rand_enum); // Output -> Option1
    print_option(&a_struct); // Output -> Option is 1

    println!("\n{:?}", a_struct);
}
