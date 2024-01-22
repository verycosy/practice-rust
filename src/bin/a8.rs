// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Lemon,
    Orange
}

struct Drink {
    flavor: Flavor,
    ounce: f64
}

fn print_drink(drink: Drink) {
    print!("flavor: ");

    match drink.flavor {
        Flavor::Lemon => println!("lemon"),
        Flavor::Orange => println!("orange")
    }

    println!("ounce: {:?}", drink.ounce);
}

fn main() {
    let drink1 = Drink {
        flavor: Flavor::Lemon,
        ounce: 3.2
    };

    let drink2 = Drink {
        flavor: Flavor::Orange,
        ounce: 2.1
    };

    print_drink(drink1);
    print_drink(drink2);
}
