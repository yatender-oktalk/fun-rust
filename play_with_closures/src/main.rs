use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 18;
    let simulated_random_number = 10;
    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_result = |num: u64| -> u32 {
        println!("Doing some expensive calculation and generating your workout plan");
        thread::sleep(Duration::from_secs(num));
        intensity
    };

    if intensity < 25 {
        let result = expensive_result(3);
        println!("Today do {} pushups", result);
        println!("After that do {} situps", result);
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated");
        } else {
            println!("Today do cardio for {} minutes", expensive_result(1));
        }
    }
}
