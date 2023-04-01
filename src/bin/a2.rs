enum Ticket {
    BackStage(f64, String),
    Vip(f64, String),
    Normal(f64),
}

fn main() {
    let tickets = vec![
        Ticket::BackStage(100.00, String::from("Adrian")),
        Ticket::Vip(65.00, String::from("Michelle")),
        Ticket::Normal(65.00),
    ];

    for tix in tickets {
        match tix {
            Ticket::BackStage(price, owner) => {
                println!(
                    "Back stage ticket price: {:?} , ticket owner: {:?}",
                    price, owner
                );
            }
            Ticket::Vip(price, owner) => {
                println!("Vip ticket price: {:?} , ticket owner: {:?}", price, owner);
            }
            Ticket::Normal(price) => {
                println!("General ticket price: {:?}", price);
            }
        }
    }
}
