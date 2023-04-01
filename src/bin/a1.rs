enum Discount {
    Percent(f32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: i32,
}

fn main() {
    let n = 3;
    match n {
        3 => println!("Three"),
        other => println!("Number: {:?}", other),
    }

    let flat = Discount::Flat(2);

    match flat {
        Discount::Flat(2) => println!("Discount 2"),
        Discount::Flat(amount) => println!("Flat discount: {:?}", amount),
        // Discount::Percent(amount) => println!("Discount percent: {:?}", amount),
        _ => (),
    }

    let cinema_ticket = Ticket {
        event: String::from("Thor Movie."),
        price: 5,
    };

    match cinema_ticket {
        Ticket { price: 5, event } => println!("Price @ 5, Event: {:?}", event),
        Ticket { price, .. } => println!("Price: {:?}", price),
    }
}
