use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 18;
    let simulated_random_number = 10;
    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today do {} pushups",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "After that do {} situps",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated");
        } else {
            println!(
                "Today do cardio for {} minutes",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Doing some expensive calculation and generating your workout plan");
    thread::sleep(Duration::from_secs(3));
    intensity
}
