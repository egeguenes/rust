enum Ticket {
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String),
}

fn main() {
    let tickets = vec![
        Ticket::Backstage(50.0, "Backstage".to_owned()),
        Ticket::Standard(12.0),
        Ticket::Vip(30.0, "Vip".to_owned()),
    ];

    for ticket in tickets {
        match ticket {
            Ticket::Backstage(price, holder) => {
                println!("Holder: {:?}, price: {:?}", price, holder)
            }
            Ticket::Standard(price) => println!("Price: {:?}", price),
            Ticket::Vip(price, holder) => println!("Holder: {:?}, price: {:?}", price, holder),
        }
    }
}
