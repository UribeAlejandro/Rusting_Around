pub fn shadowing_var(x: i32) -> String {
    let result: String = if x > 5 {
        "You pass!".to_string()
    } else {
        "You fail!".to_string()
    };

    result
}
