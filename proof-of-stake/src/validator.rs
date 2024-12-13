#[derive(Debug, Clone)]
pub struct Validator {
    pub address: String,
}

impl Validator {
    pub fn new(address: String) -> Self {
        Self { address }
    }
}
