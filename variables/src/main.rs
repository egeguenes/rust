use std::env;
use std::io;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    /*
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

    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index");

    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to read!");

    let index: usize = index.trim().parse().expect("Failed to parse!");

    let element = a[index];
    println!("The value of the element at index {index} is {element}");

    let args: Vec<String> = env::args().collect();
    let to_print = &args[1];
    echor(to_print);
    */

    let myNum = 11;
    let mut myNumMut = 11;
    println!("with ref {}", plus_ref(&myNum));
    println!("with no ref {}", plus_no_ref(myNum));
    inc_ref(&mut myNumMut);
    println!("myNumMut increment {}", myNumMut);

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");
    println!("5th fibonacci number is {}", fibo(5));

    let num = 14;
    is_copy(num);
    println!("{num}");

    let mut abc = String::from("abc");
    let a = &mut abc;
    let b = &mut abc;

    let str = String::from("Hello");
    let len = str.len();
    let str = &str[0..len];
    let str = &str[..];

    check_first_word();

    let username = String::from("username");
    let email = String::from("username@user.com");
    let user1 = build_user(username, email);
}

fn build_user(username: String, email: String) -> User {
    /*
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
    */

    // easier short hand verison
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn check_first_word() -> () {
    let str = String::from("Potato machine bool checker");
    assert_eq!("Potato", first_word(&str));
    assert_eq!("Potato", first_word(&str[..]));
    assert_eq!("Potato", first_word(&str[0..12]));
    println!("First word check!");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..];
}

fn first_word_index(s: &str) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn is_copy(num: i32) {
    println!("{num}");
}

// fib numbers are always positive, so using unsigned integer
fn fibo(n: u32) -> u64 {
    let (mut a, mut b) = (0u64, 1u64);

    for _ in 0..n {
        (a, b) = (b, a + b);
    }

    b
}

fn echor(arg: &str) {
    println!("{arg}");
}

fn plus_ref(num: &i32) -> i32 {
    num + 1
}

fn plus_no_ref(num: i32) -> i32 {
    num + 1
}

fn inc_ref(num: &mut i32) {
    *num += 1;
}
