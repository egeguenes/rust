/*
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
*/

enum IpAddr {
    //V4(String),
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    /*
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    */
    //let home = IpAddr::V4(String::from("127.0.0.1"));
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let some_number = Some(12);
    let some_char = Some('c');
    let absent_number: Option<i32> = None;

    let some_num = Some(17);
    let some_18 = plus_one(some_num);
    println!("some_num => {:?}\nsome_18 => {:?}", some_num, some_18);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(num) => Some(num + 1),
        None => None, // can leave this line empty
    }
}
