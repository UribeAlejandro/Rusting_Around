pub fn just_bool(passed: bool) -> String {
    if passed {
        "You pass!".to_string()
    } else {
        "You fail!".to_string()
    }
}

pub fn just_bool_number(number: i32) -> String {
    if number > 3 {
        "You pass!".to_string()
    } else {
        "You fail!".to_string()
    }
}
