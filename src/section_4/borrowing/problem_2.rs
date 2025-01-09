// Problem 2: Fix the code so that it compiles.

// FIXED

#[allow(unused_assignments)]
#[allow(unused_mut)]
#[allow(unused_variables)]

fn main() {
    let mut vec_1 = vec![1, 2, 3];
    let vec_2 = vec![4, 5, 6];
    let mut vec_ptr: &Vec<i32>;
    vec_ptr = &vec_1;
    println!("vec_ptr is pointing to vec_1");
    vec_ptr = &vec_2;
    println!("vec_ptr is updated and now pointing to vec_2");
}
