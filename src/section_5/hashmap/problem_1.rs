// Problem 1:
/* In this exercise, you will be working on creating a student management system
using Rust. The system should allow you to store and retrieve student information
based on their unique ID. For ease of work, the student structure is already
created in the code below

Next, create a StudentManager structure containing a field of student, which
will essentially be a hashmap where the key part will be an integer representing
unique ID of student and the value part will be the complete details of the
students contained in the student structure.

The StudentManager should implement the following methods:
1. new() -> Self: A constructor that initializes an empty student manager.

2. add_student(&mut self, student: Student) -> Result<(), String>:
Adds a student to the manager.
If the student's ID already exists, return an error message.
Otherwise, add the student to the manager and return Ok.

3. get_student(&self, id: i32) -> Option<&Student>: Retrieves a student
from the manager based on their ID.
If the student is found, return Some(student). Otherwise, return None.

Your task is to implement the StudentManager structure, and the mentioned methods.
Additionally, provide a sample usage of the student management system by adding
a few students and retrieving their information using the get_student() method.
*/

use std::collections::HashMap;

struct Student {
    id: i32,
    name: String,
    grade: u32,
}

struct StudentManager {
    studen_db: HashMap<i32, Student>,
}

impl StudentManager {
    fn new() -> Self {
        // A constructor that initializes an empty student manager.
        return Self {
            studen_db: HashMap::new(),
        };
    }

    fn add_student(&mut self, student: Student) -> Result<(), String> {
        // Adds a student to the manager.
        // If the student's ID already exists, return an error message.
        // Otherwise, add the student to the manager and return Ok.
        if let Some(_student) = self.studen_db.get(&student.id) {
            return Err("Student ID: {id} already exists!".to_string());
        } else {
            self.studen_db.insert(student.id, student);
            return Ok(());
        }
    }

    fn get_student(&self, id: i32) -> Option<&Student> {
        // Retrieves a student from the manager based on their ID.
        // If the student is found, return Some(student). Otherwise, return None.
        if let Some(_student) = self.studen_db.get(&id) {
            return self.studen_db.get(&id);
        } else {
            return None;
        }
    }
}

fn main() {
    let mut student_manager = StudentManager::new();
    let student_1 = Student {
        id: 1,
        name: "Mark".to_string(),
        grade: 50,
    };

    let student_2 = Student {
        id: 2,
        name: "John".to_string(),
        grade: 60,
    };

    let student_3 = Student {
        id: 1,
        name: "Alex".to_string(),
        grade: 60,
    };

    student_manager.add_student(student_1);
    student_manager.add_student(student_2);
    student_manager.add_student(student_3);

    for (key, value) in &student_manager.studen_db {
        println!("{key}, {0}, {1}", value.name, value.grade);
    }

    let got_student = student_manager.studen_db.get(&4);
    match got_student {
        Some(_student) => println!(
            "{0}, {1}, {2}",
            got_student.unwrap().id,
            got_student.unwrap().name,
            got_student.unwrap().grade
        ),
        None => println!("Student with ID: 4 does not exist."),
    };
}
