//Problem 1: Modify the code by eliminating all the let statements

// FIXED

struct Fruit {
    apples: i32,
    bananas: i32,
}

fn increase_fruit(fruit: Fruit) -> Fruit {
    return Fruit {
        apples: fruit.apples * 2,
        bananas: fruit.bananas * 3,
    };
}

fn new_fruit() -> Fruit {
    return Fruit {
        apples: 10,
        bananas: 5,
    };
}

fn print_fruit(fruit: Fruit) {
    println!(
        "You have {} apples and {} bananas",
        fruit.apples, fruit.bananas
    );
}

fn main() {
    print_fruit(increase_fruit(new_fruit()));
}
