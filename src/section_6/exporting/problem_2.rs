mod University {
    pub struct Student {
        pub name: String,
        pub marks: u8,
        pub grade: char,
    }

    // impl Student {
    //     pub fn new(name: String, marks: u8, grade: char) -> Student {
    //         return Student { name, marks, grade };
    //     }
    // }
}

use University::Student;

fn main() {
    let mut student_1 = Student {
        name: String::from("Alice"),
        marks: 75,
        grade: 'A',
    };

    println!("{} got {} grade", student_1.name, student_1.grade);
}
