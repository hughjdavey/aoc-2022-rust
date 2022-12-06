pub fn to_int(str: &String) -> i32 {
    return str.parse::<i32>().unwrap();
}

// thanks to https://stackoverflow.com/a/37547426
pub fn split_to_vec(str: &String, pattern: &str) -> Vec<String> {
    return str.split(pattern)
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}
