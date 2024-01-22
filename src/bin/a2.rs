// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

// * Use a function to add two numbers together
fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result
fn print_number(num: i32) {
    println!("{:?}", num);
}

fn main() {
    let num1 = 3;
    let num2 = 4;

    print_number(add(num1, num2));
}
