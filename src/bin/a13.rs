// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector

fn main() {
    let numbers = vec![10,20,30,40]; // 메인 함수 소유

    // for num in numbers { // for문 소유로 넘어갔다가 반복이 끝나면 사용불가
    for num in &numbers {
        if *num == 30 {
            println!("thirty");
        } else {
            println!("{:?}", num);
        }

        // match num {
        //     30 => println!("thirty"),
        //     _ => println!("{:?}", num)
        // }
    }

    println!("length : {:?}", numbers.len());
}
