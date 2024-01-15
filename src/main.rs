extern crate rusting_around;

fn main() {
    // Variables
    println!("{}", rusting_around::variables::immutable::hello_world());
    println!(
        "Rounded pi: {}",
        rusting_around::variables::mutable::rounding_pi()
    );
    // Control Flow
    println!(
        "{}",
        rusting_around::control_flow::conditionals::just_bool(false)
    );
    println!(
        "{}",
        rusting_around::control_flow::conditionals::just_bool(true)
    );
    println!(
        "{}",
        rusting_around::control_flow::conditionals::just_bool_number(1)
    );
}
