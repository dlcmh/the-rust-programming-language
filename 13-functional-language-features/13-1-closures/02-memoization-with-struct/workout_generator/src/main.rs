use workout_generator::workout;

fn main() {
    // calculating slowly...
    // Today, do 10 pushups!
    // Next, do 10 situps!
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    // calculating slowly...
    // Today, run for 100 minutes!
    // let simulated_user_specified_value = 100;
    // let simulated_random_number = 7;

    // Take a break today! Remeber to stay hydrated!
    // let simulated_user_specified_value = 100;
    // let simulated_random_number = 3;

    workout::generate_workout(simulated_user_specified_value, simulated_random_number);
}
