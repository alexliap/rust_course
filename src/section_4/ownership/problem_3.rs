// Problem 3: Fix the code so that it compiles.

// FIXED

#[allow(unused_variables)]

fn main() {
    let str1 = generate_string();
    let str2 = str1;
}

fn generate_string() -> String {
    let some_string = String::from("I will generate a string");
    return some_string;
}
