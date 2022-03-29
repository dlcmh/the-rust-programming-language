use workout_generator::workout;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 3;

    workout::generate_workout(simulated_user_specified_value, simulated_random_number);
}
