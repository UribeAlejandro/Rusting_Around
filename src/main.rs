extern crate rusting_around;

fn main() {
    // Variables
    println!("{}", rusting_around::variables::immutable::hello_world());
    println!(
        "Rounded pi: {}",
        rusting_around::variables::mutable::rounding_pi()
    );
}
