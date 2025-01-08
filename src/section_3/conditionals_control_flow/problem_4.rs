// Problem 4:

/*
A palindrome is a word, verse, or sentence that reads the same backward or forward,
such as 'Able was I ere I saw Elba,' or a number like 1881.

Write a function named is_palindrome() that checks whether a given string is a palindrome or not.
The function should take a string as input and return a boolean value indicating whether the string is a palindrome or not.
*/

fn main() {
    let input = String::from("1221");
    println!(
        "It is {:?} that the given string is palindrome",
        is_palindrome(input)
    );
}

fn is_palindrome(input: String) -> bool {
    let palindrome: bool;

    let length: usize = input.len();

    let mut mirror: String = "".to_string();

    for i in (0..length).rev() {
        mirror.push(input.chars().nth(i).unwrap())
    }

    if mirror == input {
        palindrome = true;
    } else {
        palindrome = false;
    }

    println!("{mirror}");

    return palindrome;
}
