// Problem 1: Fix the code below

// FIXED

fn main() {
    let mut some_vec = vec![1, 2, 3];
    let first = get_first_element(&some_vec);
    println!("The first number is: {}", first);
    some_vec.push(4);

    /*
    The problem with borrowing arises when we attempt to modify the some_vec vector
    after obtaining an immutable reference to its first element.
    This situation violates Rust's borrowing rules, which dictate that we cannot modify a variable
    while immutable references to it are still in scope.

    This rule ensures the safety and integrity of data in Rust,
    preventing potential conflicts and data races
    */
}

fn get_first_element(num_vec: &Vec<i32>) -> &i32 {
    &num_vec[0]
}
