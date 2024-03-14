// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age: i32,
    name: String,
    color: String
}

fn print_info(name: &str, color: &str) {
    println!("Name: {:?}, Color: {:?}", name, color);
}

fn main() {
    let people = vec![
        Person {
            age: 12,
            name: String::from("Alice"),
            color: String::from("Red")
        },
        Person {
            age: 10,
            name: String::from("Bob"),
            color: String::from("Blue")
        },
        Person {
            age: 2,
            name: String::from("Chris"),
            color: String::from("Green")
        },
        Person {
            age: 14,
            name: String::from("David"),
            color: String::from("White")
        },
    ];

    for person in people {
        if person.age <= 10 {
            print_info(&person.name, &person.color);
        }
    }
}
