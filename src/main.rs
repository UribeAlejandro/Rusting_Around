extern crate rusting_around;

fn main() {
    // Variables
    println!("Variables:");
    println!("{}", rusting_around::variables::immutable::hello_world());
    println!(
        "Rounded pi: {}",
        rusting_around::variables::mutable::rounding_pi()
    );
    println!(
        "Shadowing var: {}",
        rusting_around::variables::shadowing::shadowing_var(5)
    );

    // Control Flow
    println!("Control Flow:");
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

    // Loops
    println!("Loops:");
    rusting_around::control_flow::loops::basic_loop();

    // Functions
}
