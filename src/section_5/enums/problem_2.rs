// Problem:
/* You are tasked with implementing a library management system using Rust.
Your goal is to design a program that can handle books and magazines.
To fulfill the requirements, follow the steps below:

Create a structure called Item with the following fields:
id: An integer representing the unique identifier of the item.
title: A string representing the title of the item.
year: An integer representing the publication year of the item.
type: an enumeration type. The details are coming below.

Create an enumeration called ItemType with two variants:
Book: Represents a book.
Magazine: Represents a magazine.

Implement a function called display_item_info() that takes an Item as input
and displays its information. The function should output
the item's ID, title, publication year, and type (book or magazine).
*/

enum ItemType {
    Book,
    Magazine,
}

struct Item {
    id: u32,
    title: String,
    year: u32,
    item_type: ItemType,
}
impl Item {
    fn display_item_info(&self) {
        let item: String = match self.item_type {
            ItemType::Book => "book".to_string(),
            ItemType::Magazine => "magazine".to_string(),
        };

        println!("{0}, {1}, {2}, {3}", self.id, self.title, self.year, item)
    }
}

fn main() {
    let data = Item {
        id: 12,
        title: "Title".to_string(),
        year: 2000,
        item_type: ItemType::Book,
    };

    data.display_item_info()
}
