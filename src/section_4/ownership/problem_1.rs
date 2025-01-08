// Problem 1: Fix the code below so that it compiles

// FIXED

fn main() {
    let s1: String = String::from("this is me, ");
    let s2: &str = "Nouman";
    some_function(s1.clone(), s2);
    println!("{} {}", s1, s2);
}

fn some_function(a1: String, a2: &str) {
    println!("{} {}", a1, a2);
}
