// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn print(is_lt_100: bool) {
    match is_lt_100 {
        true => println!("its small"),
        false => println!("its big")
    };
}

fn main() {
    let num = 99;
    let is_lt_100 = if num < 100 {
        true
    } else {
        false
    };

    print(is_lt_100);
}
