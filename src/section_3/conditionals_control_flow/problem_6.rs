// Problem 6: Write a function that implements the logic,
// 'You can see the movie if you are 17 or older, or if you are 13 or older and have a parent's permission.'
// Note: This means that if you 17 or older, you implicitly have permission.

fn can_see_movie(age: i32, permission: bool) -> bool {
    let see_movie: bool;
    if age >= 17 {
        see_movie = true;
    } else if (age >= 13) && (permission == true) {
        see_movie = true;
    } else {
        see_movie = false;
    }

    return see_movie;
}

fn main() {
    println!(
        "John who is 18, can see the movie: {}",
        can_see_movie(17, true)
    );
}
