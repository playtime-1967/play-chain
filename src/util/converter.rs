use anyhow::{Context, Ok, Result};

pub fn convert_to_u16(input: String) -> Result<u16> {
    input
        .parse::<u16>()
        .context("Failed to parse the input as a valid u16")
}

pub fn convert_vec_of_str_to_vec_of_string(vec: Vec<&str>) -> Vec<String> {
    vec.iter().map(|&s| s.to_string()).collect()
}
