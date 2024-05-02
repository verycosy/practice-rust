// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker: Option<i32>
}

fn main() {
    let students: Vec<Student> = vec![Student {
        name: "mark".to_owned(),
        locker: None
    }, Student {
        name: "tom".to_owned(),
        locker: Some(2)
    }];

    for student in students {
        match student {
            Student { name, locker: None } => println!("{:?} has no locker", name),
            Student { name, locker: Some(locker) } => println!("{:?} has locker num {:?}", name, locker),
        }
    }

}
