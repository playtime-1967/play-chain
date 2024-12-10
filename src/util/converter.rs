use anyhow::{Context, Ok, Result};

fn convert_to_u16(input: String) -> Result<u16> {
    input
        .parse::<u16>()
        .context("Failed to parse the input as a valid u16")
}
