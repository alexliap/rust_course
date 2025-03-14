// Problem 3:

/*
This question involves writing code to analyze the production of an assembly line in a car factory.
The assembly line has different speeds, ranging from 0 (off) to 10 (maximum).
At the lowest speed of 1, the assembly line produces a total of 221 cars per hour.
The production rate increases linearly with the speed,
meaning that a speed of 4 produces 4 * 221 = 884 cars per hour.

However, higher speeds increase the likelihood of producing faulty cars that need to be discarded.
The success rate depends on the speed, as shown in the table below:
· Speeds 1 to 4: 100% success rate.
· Speeds 5 to 8: 90% success rate.
· Speeds 9 and 10: 77% success rate.

You need to write two functions:
1. The first function, total_production(), calculates the total number of cars successfully produced without faults
within a specified time given in hours. The function takes the number of hours and speed as input and returns the number
of cars successfully produced.

2. The second function, cars_produced_per_minute(), calculates the number of cars successfully produced per minute.
The function takes the number of hours and speed as input and returns the number of cars produced per minute.
Write the code for both functions based on the provided specifications.
*/

fn main() {
    println!("{}", total_production(6, 5) as i32); // to round the values we use i32. just ignore for mow
    println!("{}", cars_produced_per_minutes(6, 5) as i32); // to round the values we use i32. just ignore for mow
}

fn total_production(hours: u8, speed: u8) -> f32 {
    let total_production: f32;
    let mut success_rate: f32 = 0.;

    match speed {
        1..=4 => success_rate = 1.0,
        5..=8 => success_rate = 0.9,
        9..=10 => success_rate = 0.77,
        _ => (),
    }

    total_production = hours as f32 * (speed as f32 * 221. * success_rate);

    return total_production;
}

fn cars_produced_per_minutes(hours: u8, speed: u8) -> f32 {
    return total_production(hours, speed) / (hours as f32 * 60.);
}
