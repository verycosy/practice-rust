// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Ticket {
    Backstage(i32, String),
    Vip(i32, String),
    Standard(i32)
}

fn main() {
    let tickets: Vec<Ticket> = vec![
        Ticket::Backstage(3, "backstage".to_owned()),
        Ticket::Vip(4, "vip".to_owned()),
        Ticket::Standard(2),
    ];

    for ticket in tickets {
        match ticket {
            Ticket::Standard(price) => println!("Standard {:?}", price),
            Ticket::Backstage(price, holder) => println!("Backstage {:?} {:?}", price, holder),
            Ticket::Vip(price, holder) => println!("Vip {:?} {:?}", price, holder),
        }
    }
}
