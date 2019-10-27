use std::thread;
use std::time::Duration;

fn simulated_expansive_calculation(itensity: u32) -> u32 {
    println!("calculating...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(itensity: u32, random_number: u32) {
    let expensive_closue = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closue(itensity)
        );
        println!(
            "Next, do {} situps",
            expensive_closue(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Rememeber to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closue(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simuleted_random_number = 7;

    generate_workout(
        simuluted_user_specified_value,
        simulated_random_number
    );
}
