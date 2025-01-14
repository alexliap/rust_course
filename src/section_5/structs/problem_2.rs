// Problem 2: Fix the code by only changing the indicated line.

// FIXED

struct Book {
    title: String,
}

fn main() {
    let some_book = Book {
        title: String::from("The Rust Programming Language"),
    };
    print_book(&some_book.title);
    println!("Book name: {}", some_book.title);
}

fn print_book(book_name: &String) {
    println!("Book: {}", book_name);
}
