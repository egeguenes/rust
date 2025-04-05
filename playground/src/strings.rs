struct Name {
    first_name: String,
    last_name: String,
}

impl Name {
    fn new(first: &str, last: &str) -> Self {
        Name {
            first_name: first.to_owned(),
            last_name: last.to_owned(),
        }
    }
}

fn main() {
    let first_name = String::from("First");
    let last_name = "Lastname".to_owned();

    let new_name = Name {
        first_name: first_name,
        last_name: last_name,
    };

    let new_name_2 = Name::new(&first_name, &last_name);

    println!(
        "First names: {:?} , {:?}",
        new_name.first_name, new_name_2.first_name
    );
    println!(
        "Last names: {:?} , {:?}",
        new_name.last_name, new_name_2.last_name
    );
}
