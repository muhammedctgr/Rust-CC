pub fn num_to_string(num: u32) -> String {
    match num {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "other",
    }
    .to_string()
}


