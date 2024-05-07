// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

struct Customer {
    age: i32
}

fn can_purchase(customer: &Customer) -> Result<(), String> {
    if customer.age >= 21 {
        return Ok(())
    }

    Err("Can not purchase: too young ".to_owned())
}

fn main() {
    let customer1 = Customer {
        age: 21
    };

    let customer2 = Customer {
        age: 20
    };

    let result1 = can_purchase(&customer1);
    println!("{:?}", result1);

    let result2 = can_purchase(&customer2);
    println!("{:?}", result2);
}
